use std::rc::Rc;
use object_pool::Pool;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::VecDeque;
use godot::obj::{Bounds, bounds, Gd, GodotClass};

pub trait ReuseObject {
    fn init(&mut self);
    fn prepare(&mut self);
    fn recycle(&mut self);
}

impl <C> ReuseObject for Gd<C>
where C: ReuseObject + GodotClass + Bounds<Declarer = bounds::DeclUser>, {
    fn init(&mut self) {
        self.bind_mut().init();
    }

    fn prepare(&mut self) {
        self.bind_mut().prepare();
    }

    fn recycle(&mut self) {
        self.bind_mut().recycle();
    }
}

pub struct GdPool<R: ReuseObject> {
    capacity: usize,
    pool: Rc<Pool<R>>,
    buffer: RefCell<VecDeque<Rc<Reuse<R>>>>,
}

pub struct Reuse<R: ReuseObject> {
    pool: Rc<Pool<R>>,
    obj: RefCell<Option<R>>
}

impl <R: ReuseObject> Reuse<R> {
    pub fn borrow(&self) -> Ref<R> {
        Ref::map(self.obj.borrow(), |obj| {
            obj.as_ref().expect("impossible")
        })
    }

    pub fn borrow_mut(&self) -> RefMut<R> {
        RefMut::map(self.obj.borrow_mut(), |obj| {
            obj.as_mut().expect("impossible")
        })
    }
}


impl <R> Drop for Reuse<R>
where R: ReuseObject {
    fn drop(&mut self) {
        if let Some(object) = self.obj.borrow_mut().take() {
            tracing::debug!("Dropping obj");
            self.pool.attach(object);
        }
    }
}

impl <R: ReuseObject> GdPool<R> {
    pub fn new<F>(capacity: usize, init: F) -> Self
    where F: Fn() -> R {
        GdPool {
            capacity,
            pool: Rc::new(Pool::new(
                capacity * 2,
                || {
                    let mut obj = init.call(());
                    ReuseObject::init(&mut obj);
                    obj
                }
            )),
            buffer: RefCell::new(VecDeque::with_capacity(capacity * 2)),
        }
    }

    pub fn get(&self) -> Option<Rc<Reuse<R>>> {
        let object = self.pool.try_pull()?;
        let (_, mut obj) = object.detach();
        obj.prepare();
        tracing::debug!("object prepare");
        let reuse = Rc::new(Reuse {
            pool: self.pool.clone(),
            obj: RefCell::new(Some(obj)),
        });
        let mut buf = self.buffer.borrow_mut();
        buf.push_front(reuse.clone());
        if buf.len() > self.capacity {
            tracing::debug!("buffer full {}, drop older object", buf.len());
            let drain: Vec<Rc<Reuse<R>>> = buf
                .drain(self.capacity..)
                .collect();
            for reuse in drain {
                tracing::debug!("destroy object");
                reuse.obj
                    .borrow_mut()
                    .as_mut()
                    .expect("impossible")
                    .recycle()
            }
        }

        Some(reuse)
    }
}
