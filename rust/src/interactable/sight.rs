use godot::engine::{Area2D, CollisionShape2D, IArea2D, Timer};
use godot::prelude::*;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::TryRecvError;
use crate::runtime::RT;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct SightArea2D {
    rx: tokio::sync::mpsc::Receiver<bool>,
    base: Base<Area2D>,
}

#[godot_api]
impl SightArea2D {
    #[signal]
    fn character_in_sight(character: Gd<Area2D>);

    #[func]
    fn on_area_entered(&mut self, body: Gd<Area2D>) {
        self.base_mut().emit_signal("character_in_sight".into(), &[body.to_variant()]);
    }

    #[func]
    fn on_timer_timeout(&mut self) {
        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .set_disabled(false);
    }
}

#[godot_api]
impl IArea2D for SightArea2D {
    fn init(base: Base<Area2D>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel::<bool>(16);
        RT.spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                if let Err(_) = tx.send(false).await {
                    break
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
                if let Err(_) = tx.send(true).await {
                    break
                }
            }
            tracing::debug!("SightArea2D tokio task finished");
        });
        SightArea2D {
            rx,
            base,
        }
    }

    fn ready(&mut self) {
        let listener = self.base().callable("on_area_entered");
        self.base_mut().connect("area_entered".into(), listener);

        let mut timer = self.base().get_node_as::<Timer>("Timer");
        let timeout_listener = self.base().callable("on_timer_timeout");
        timer.connect("timeout".into(), timeout_listener);
    }

    fn process(&mut self, delta: f64) {
        let Ok(flag) = self.rx.try_recv() else {
            return
        };

        let mut base = self.base_mut();
        let mut coll = base.get_node_as::<CollisionShape2D>("CollisionShape2D");
        coll.set_disabled(flag);
        tracing::debug!("flag: {}", flag);
    }
}
