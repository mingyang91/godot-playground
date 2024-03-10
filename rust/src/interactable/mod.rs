use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Mutex;
use godot::obj::Bounds;
use godot::obj::bounds::DeclUser;

use godot::prelude::{Gd, GodotClass, Inherits, Node2D};
use lazy_static::lazy_static;
use crate::interactable::tree::PineTree;

pub mod tree;


pub trait Caster<T>
where Self: Send {
    fn name(&self) -> String;
    fn cast(&self, gd: Gd<Node2D>) -> T;
}


pub type BoxInteractWith = Box<dyn InteractWith<PineTree>>;

lazy_static! {
    static ref REGISTER: Mutex<HashMap<String, Box<dyn Caster<BoxInteractWith>>>> =
        Mutex::new(HashMap::new());
}

pub fn register<C>(name: &str, caster: C)
where C: Caster<BoxInteractWith> + Send + 'static {
    let mut reg = REGISTER.lock().unwrap();
    reg.insert(name.to_string(), Box::new(caster));
}

pub fn is_registered(name: &str) -> bool {
    let reg = REGISTER.lock().unwrap();
    reg.contains_key(name)
}

pub fn cast<T>(node: Gd<Node2D>) -> Result<Gd<T>, Gd<Node2D>>
where T: GodotClass + Inherits<Node2D> {
    let name = node.get_name().to_string();
    if is_registered(&name) {
        let node = node.try_cast::<T>();
        return node
    }
    Err(node)
}

trait MaybeImplInteractWith<T>
where Self: Sized {
    fn get_dyn(self) -> Result<Box<dyn InteractWith<T>>, Self>;
}

impl MaybeImplInteractWith<PineTree> for Gd<Node2D> {
    fn get_dyn(self) -> Result<Box<dyn InteractWith<PineTree>>, Self> {
        let register = REGISTER.lock().expect("lock");
        let name = self.get_name().to_string();
        let Some(caster) = register.get(&name) else {
            return Err(self);
        };
        Ok(caster.cast(self))
    }
}

pub trait InteractWith<T> {
    fn interact(&mut self, with: &mut T);
}

impl <T, F> InteractWith<T> for Gd<F>
where F: GodotClass + InteractWith<T> + Bounds<Declarer = DeclUser> {
    fn interact(&mut self, with: &mut T) {
        self.clone().cast::<F>().bind_mut().interact(with);
    }
}
