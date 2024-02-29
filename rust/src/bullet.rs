use godot::engine::{Area2D, IArea2D, Timer};
use godot::prelude::*;
use crate::pool::ReuseObject;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Init,
    Flying,
    Exploding,
    Inactive,
}


#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Bullet {
    #[export]
    velocity: f32,
    dir: Vector2,
    state: State,

    base: Base<Area2D>,
}

impl ReuseObject for Bullet {
    fn init(&mut self) {

    }

    fn prepare(&mut self) {
        if self.state != State::Init {
            self.state = State::Init;
            self.base_mut().set_process(true);
            self.base_mut().set_physics_process(true);
            self.base_mut().show();
        }
    }

    fn recycle(&mut self) {
        if self.state != State::Inactive {
            self.state = State::Inactive;
            self.base_mut().hide();
            self.base_mut().set_process(false);
            self.base_mut().set_physics_process(false);
        }
    }
}

#[godot_api]
impl Bullet {
    #[signal]
    fn shot(start: Vector2, dir: Vector2);

    #[func]
    fn on_body_entered(&mut self, _body: Gd<Area2D>) {
    }

    #[func]
    fn on_visibility_screen_exited(&mut self) {
        tracing::info!("Bullet exited screen");
        self.recycle();
    }

    #[func]
    pub fn shot(&mut self, start: Vector2, dir: Vector2) {
        if self.state != State::Flying {
            self.state = State::Flying;
            self.base_mut().set_global_position(start);
            self.base_mut().set_rotation(dir.angle());
            self.dir = dir;
            self.base_mut().get_node_as::<Timer>("Timer").start();
        }
    }
}


#[godot_api]
impl IArea2D for Bullet {
    fn init(base: Base<Self::Base>) -> Self {
        Bullet {
            velocity: 10f32,
            dir: Vector2::new(0f32, 1f32),
            state: State::Init,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state == State::Flying {
            let position = self.base().get_global_position();
            let vec2 = self.dir * self.velocity * delta as f32;
            self.base_mut().set_global_position(position + vec2);
        }
    }

    fn ready(&mut self) {
    }
}