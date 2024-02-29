use std::cell::{OnceCell};
use std::f32::consts::PI;
use std::rc::Rc;
use godot::engine::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, PhysicsBody2D};
use godot::prelude::*;
use crate::bullet::Bullet;
use crate::pool::{GdPool, Reuse};

struct CoolDown {
    time: f64,
    elapsed: f64,
}

impl CoolDown {
    fn new(time: f64) -> Self {
        CoolDown {
            time,
            elapsed: 0.0,
        }
    }

    fn update(&mut self, delta: f64) {
        self.elapsed += delta;
    }

    fn reset(&mut self) {
        self.elapsed = 0.0;
    }

    fn ready(&self) -> bool {
        self.elapsed >= self.time
    }
}

struct FloatingGun {
    cd: CoolDown,
    period: f64,
    angle: f64,
}

impl FloatingGun {
    fn new(cd: f64, period: f64, start_angel: f64) -> Self {
        FloatingGun {
            cd: CoolDown::new(cd),
            period,
            angle: start_angel,
        }
    }

    fn update(&mut self, delta: f64) {
        self.cd.update(delta);
        self.angle += delta * self.period;
    }

    fn reset(&mut self) {
        self.cd.reset();
    }

    fn ready(&self) -> bool {
        self.cd.ready()
    }
}

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    speed: real,
    screen_size: Vector2,
    bullet_pool: OnceCell<GdPool<Gd<Bullet>>>,
    attach_cd: CoolDown,
    floating_guns: Vec<FloatingGun>,

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
        let gun_nums = 4;
        let mut floating_guns_1: Vec<_> = (0..gun_nums)
            .map(|i| FloatingGun::new(0.1, 1.0, (PI as f64 * 2.0 / gun_nums as f64) * i as f64))
            .collect();
        let floating_guns_2: Vec<_> = (0..gun_nums)
            .map(|i| FloatingGun::new(0.1, -1.0, (PI as f64 * 2.0 / gun_nums as f64) * i as f64))
            .collect();
        floating_guns_1.extend(floating_guns_2);
        Player {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
            floating_guns: floating_guns_1,
            bullet_pool: OnceCell::new(),
            attach_cd: CoolDown::new(0.1),
            base,
        }
    }


    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        let bullet_scene = load::<PackedScene>("res://Bullet.tscn");
        let base = self.base().clone();
        let pool = GdPool::new(1000, || {
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

        if input.is_action_pressed("attack".into()) {
            for g_index in 0..self.floating_guns.len() {
                if self.floating_guns[g_index].ready() {
                    self.floating_guns[g_index].reset();
                    let angle = self.floating_guns[g_index].angle;
                    let bullet = self.create_bullet();
                    let start = self.base().get_global_position();
                    let mut bullet = bullet.borrow_mut();
                    let dir = Vector2::new(angle.cos() as real, angle.sin() as real);
                    bullet.bind_mut().shot(start, dir);
                    tracing::debug!("bullet shot");
                } else {
                    self.floating_guns[g_index].update(delta);
                }
            }
            // if self.attach_cd.ready() {
            //     self.attach_cd.reset();
            //     let start = self.base().get_global_position();
            //     let bullet = self.create_bullet();
            //     let mut bullet = bullet.borrow_mut();
            //     let dir = Vector2::new(0.0, -1.0);
            //     bullet.bind_mut().shot(start, dir);
            //     tracing::debug!("bullet shot");
            // } else {
            //     self.attach_cd.update(delta);
            // }
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
