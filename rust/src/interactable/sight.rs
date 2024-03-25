use std::borrow::BorrowMut;
use godot::engine::{Area2D, CollisionShape2D, IArea2D, Timer};
use godot::prelude::*;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::TryRecvError;
use crate::runtime::RT;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct SightArea2D {
    rx: tokio::sync::mpsc::Receiver<bool>,
    overlapping_areas: Vec<Gd<Area2D>>,
    base: Base<Area2D>,
}

#[godot_api]
impl SightArea2D {
    fn update(&mut self) {
        let overlaps = self.base_mut().get_overlapping_areas();
        self.overlapping_areas = overlaps.iter_shared().collect();
    }

    pub fn get_overlapping_areas(&self) -> &[Gd<Area2D>] {
        &self.overlapping_areas
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

                tokio::time::sleep(tokio::time::Duration::from_millis(34)).await;
                if let Err(_) = tx.send(true).await {
                    break
                }
            }
            tracing::debug!("SightArea2D tokio task finished");
        });
        SightArea2D {
            rx,
            overlapping_areas: vec![],
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let Ok(flag) = self.rx.try_recv() else {
            return
        };

        let mut coll = self.base_mut().get_node_as::<CollisionShape2D>("CollisionShape2D");

        coll.set_disabled(flag);
        self.update();
    }
}
