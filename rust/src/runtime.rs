use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::OnceLock;
use std::thread;
use std::thread::{sleep};
use actix::{System, SystemRunner};
use actix_rt::Arbiter;
use godot::log::{godot_error, godot_script_error};
use lazy_static::lazy_static;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

lazy_static! {
    pub static ref RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new()
        .expect("Failed to create tokio runtime");
}

static RUNTIME: OnceLock<ActorSystemRef> = OnceLock::new();
type Task = Pin<Box<dyn Future<Output = ()> + Send + Sync>>;

pub fn spawn(task: Task) {
    if let Err(err) = get_actor_system_ref().tx.send(task) {
        tracing::error!("Failed to start actor system: {}", err);
    }
}

pub fn get_actor_system_ref() -> &'static ActorSystemRef {
    RUNTIME.get_or_init(|| {
        tracing::info!("init actor system");
        let (tx, mut rx) = unbounded_channel();

        let join_handle = thread::Builder::new()
            .name("actor".to_string())
            .spawn(move || {
                tracing::info!("start actor system thread");
                System::with_tokio_rt(||
                    tokio::runtime::Runtime::new()
                        .expect("Failed to create tokio runtime")
                ).block_on(async {
                    let arbiter = Arbiter::new();
                    arbiter
                        .spawn(async move {
                            while let Some(task) = rx.recv().await {
                                tokio::spawn(task);
                            }
                            tracing::info!("actor system stopped");
                        });
                });
            })
            .expect("Failed to start actor system thread");

        ActorSystemRef { join_handle, tx }
    })
}

#[derive(Debug)]
pub struct ActorSystemRef {
    join_handle: thread::JoinHandle<()>,
    tx: UnboundedSender<Task>
}

#[cfg(test)]
mod test {
    struct Counter {
        val: i32
    }

    impl Counter {
        fn set(&mut self, new_val: i32) {
            self.val = new_val
        }

        fn get(&self) -> i32 {
            self.val
        }
    }

    struct State {
        counter: Counter
    }

    impl State {
        fn get_counter(&mut self) -> &mut Counter {
            &mut self.counter
        }
    }

    #[test]
    fn test() {
        let mut state = State { counter: Counter { val: 0 } };
        // state.get_counter().set(state.get_counter().get() + 1);
    }
}
