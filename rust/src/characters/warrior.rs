use std::cell::OnceCell;
use godot::engine::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Label};
use godot::prelude::*;
use rand::Rng;
use crate::characters::common::{Action, AttackCoolDown, FaceDirection};

#[derive(Debug)]
struct State {
	hp: i32,
	action: Action,
	face_direction: FaceDirection,
	attack_cool_down: AttackCoolDown,
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Warrior {
	#[var]
	gravity: real,
	#[var]
	speed: real,
	state: State,
	// animated_sprite2d: OnceCell<Gd<AnimatedSprite2D>>,
	base: Base<CharacterBody2D>,
}


#[godot_api]
impl Warrior {
	#[func]
	fn on_animation_finished(&mut self) {
	}
}

#[godot_api]
impl ICharacterBody2D for Warrior {
	fn init(base: Base<CharacterBody2D>) -> Self {
		Warrior {
			gravity: 100 as real,
			speed: 100 as real,
			state: State {
				hp: 100,
				action: Action::Idle,
				face_direction: FaceDirection::Right,
				attack_cool_down: AttackCoolDown::new(1.0),
			},
			base,
		}
	}

	fn ready(&mut self) {
		let mut anime = self.base_mut()
			.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
		anime.play();
	}

	fn process(&mut self, delta: f64) {
		self.state.attack_cool_down.update(delta);
		let mut debug = self.base().get_node_as::<Label>("Debug");
		debug.set_text(format!("state: {:?}", self.state).into());
		self.process_input()
	}
}

trait InputAction {
	fn is_accept_input(&self) -> bool;
	fn attack_pressed(&mut self);

	fn process_input(&mut self);
}

impl InputAction for Warrior {
	fn is_accept_input(&self) -> bool {
		match self.state.action {
			Action::Attack => false,
			Action::Die => false,
			_ => true
		}
	}

	fn attack_pressed(&mut self) {
		if self.state.attack_cool_down.ready() {
			self.state.attack_cool_down.reset();
			self.state.action = Action::Attack;
		}
	}

	fn process_input(&mut self) {
		if !self.is_accept_input() {
			return
		}
		let input = Input::singleton();
		if input.is_action_pressed("attack".into()) {
			self.attack_pressed();
			return
		} else {
			let mut velocity = Vector2::ZERO;
			if input.is_action_pressed("move_right".into()) {
				self.state.face_direction = FaceDirection::Right;
				velocity += Vector2::RIGHT;
			}
			if input.is_action_pressed("move_left".into()) {
				self.state.face_direction = FaceDirection::Left;
				velocity += Vector2::LEFT;
			}
			if input.is_action_pressed("move_down".into()) {
				velocity += Vector2::DOWN;
			}
			if input.is_action_pressed("move_up".into()) {
				velocity += Vector2::UP;
			}

			if velocity.length() > 0.0 {
				velocity = velocity.normalized() * self.speed;
				self.state.action = Action::Walk;
			} else {
				self.state.action = Action::Idle;
			}

			self.base_mut().set_velocity(velocity);
			self.base_mut().move_and_slide();
		}
	}
}