use std::cell::{OnceCell};
use std::rc::Rc;
use godot::engine::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, PhysicsBody2D};
use godot::prelude::*;
use crate::bullet::Bullet;
use crate::pool::{GdPool, Reuse};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    speed: real,
    screen_size: Vector2,
    bullet_pool: OnceCell<GdPool<Gd<Bullet>>>,

    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[signal]
    fn shot(start: Vector2, dir: Vector2);

    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
        self.base_mut().hide();
        self.base_mut().emit_signal("hit".into(), &[]);

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_deferred("disabled".into(), true.to_variant());
    }

    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }

    fn create_bullet(&mut self) -> Rc<Reuse<Gd<Bullet>>> {
        let pool = self.bullet_pool
            .get_mut()
            .expect("uninitialized bullet pool");
        let bullet = pool.get()
            .expect("pool exhausted")
            .clone();
        return bullet
    }
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        tracing::error!("Player thread: {:?}", std::thread::current().id());
        Player {
            speed: 400.0,
            bullet_pool: OnceCell::new(),
            screen_size: Vector2::new(0.0, 0.0),
            base,
        }
    }


    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        let bullet_scene = load::<PackedScene>("res://Bullet.tscn");
        let base = self.base().clone();
        let pool = GdPool::new(10, || {
            tracing::debug!("Instantiate bullet");
            let bullet = bullet_scene.instantiate_as::<Bullet>();
            let Some(mut parent) = base.get_parent() else {
                tracing::error!("Failed to get parent of bullet");
                unreachable!("Failed to get parent of bullet");
            };
            parent.call_deferred("add_child".into(), &[bullet.clone().to_variant()]);
            bullet
        });
        if let Err(_) = self.bullet_pool.set(pool) {
            tracing::error!("Failed to create bullet pool!");
        }
        self.base_mut().hide();
    }

    fn process(&mut self, delta: f64) {
        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mut velocity = Vector2::new(0.0, 0.0);

        // Note: exact=false by default, in Rust we have to provide it explicitly
        let input = Input::singleton();
        if input.is_action_pressed("move_right".into()) {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left".into()) {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("move_down".into()) {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("move_up".into()) {
            velocity += Vector2::UP;
        }

        if input.is_action_just_released("attack".into()) {
            let start = self.base().get_global_position();
            let bullet = self.create_bullet();
            let mut bullet = bullet.borrow_mut();
            bullet.bind_mut().shot(start, velocity);
            tracing::debug!("bullet shot");
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;

            let animation;

            if velocity.x != 0.0 {
                animation = "right";

                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0)
            } else {
                animation = "up";

                animated_sprite.set_flip_v(velocity.y > 0.0)
            }

            animated_sprite.play_ex().name(animation.into()).done();
        } else {
            animated_sprite.stop();
        }

        let change = velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }
}
