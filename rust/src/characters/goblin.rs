use std::cell::OnceCell;
use godot::engine::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Label};
use godot::prelude::*;
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
pub struct Goblin {
	#[var]
	gravity: real,
	#[var]
	speed: real,
	state: State,
	animated_sprite2d: OnceCell<Gd<AnimatedSprite2D>>,
	base: Base<CharacterBody2D>,
}


#[godot_api]
impl Goblin {
	#[func]
	fn on_animation_finished(&mut self) {
		match self.state.action {
			Action::Attack => self.transition_to_idle(),
			Action::Die => {}
			_ => {}
		}
	}

	fn get_animated_sprite2d(&self) -> &Gd<AnimatedSprite2D> {
		self.animated_sprite2d.get().expect("AnimatedSprite2D is not initialized")
	}

	fn get_animated_sprite2d_mut(&mut self) -> &mut Gd<AnimatedSprite2D> {
		self.animated_sprite2d.get_mut().expect("AnimatedSprite2D is not initialized")
	}

	fn transition_to_idle(&mut self) {
		self.state.action = Action::Idle;
		match self.state.face_direction {
			FaceDirection::Left => self.left_idle(),
			FaceDirection::Right => self.right_idle(),
		}
	}

	fn transition_to_walk(&mut self) {
		self.state.action = Action::Walk;
		match self.state.face_direction {
			FaceDirection::Left => self.left_walk(),
			FaceDirection::Right => self.right_walk(),
		}
	}

	fn right_idle(&mut self) {
		self.get_animated_sprite2d_mut()
			.set_flip_h(false);
		self.get_animated_sprite2d_mut()
			.play_ex()
			.name("right_idle".into())
			.done();
	}

	fn left_idle(&mut self) {
		self.get_animated_sprite2d_mut()
			.set_flip_h(true);
		self.get_animated_sprite2d_mut()
			.play_ex()
			.name("right_idle".into())
			.done();
	}

	fn right_walk(&mut self) {
		self.get_animated_sprite2d_mut()
			.set_flip_h(false);
		self.get_animated_sprite2d_mut()
			.play_ex()
			.name("right_walk".into())
			.done();
	}

	fn left_walk(&mut self) {
		self.get_animated_sprite2d_mut()
			.set_flip_h(true);
		self.get_animated_sprite2d_mut()
			.play_ex()
			.name("right_walk".into())
			.done();
	}

	fn right_attack(&mut self) {
		self.get_animated_sprite2d_mut()
			.set_flip_h(false);
		self.get_animated_sprite2d_mut()
			.play_ex()
			.name("right_attack".into())
			.done();
	}

	fn left_attack(&mut self) {
		self.get_animated_sprite2d_mut()
			.set_flip_h(true);
		self.get_animated_sprite2d_mut()
			.play_ex()
			.name("right_attack".into())
			.done();
	}

	fn up_attack(&mut self) {
		self.get_animated_sprite2d_mut()
			.set_flip_v(false);
		self.get_animated_sprite2d_mut()
			.play_ex()
			.name("up_attack".into())
			.done();
	}

	fn down_attack(&mut self) {
		self.get_animated_sprite2d_mut()
			.set_flip_v(true);
		self.get_animated_sprite2d_mut()
			.play_ex()
			.name("down_attack".into())
			.done();
	}
}

#[godot_api]
impl ICharacterBody2D for Goblin {
	fn init(base: Base<CharacterBody2D>) -> Self {
		Goblin {
			gravity: 100 as real,
			speed: 100 as real,
			state: State {
				hp: 100,
				action: Action::Idle,
				face_direction: FaceDirection::Right,
				attack_cool_down: AttackCoolDown::new(1.0),
			},
			animated_sprite2d: OnceCell::new(),
			base,
		}
	}

	fn ready(&mut self) {
		let mut anime = self.base_mut()
			.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
		anime.play();
		self.animated_sprite2d.set(anime).expect("AnimatedSprite2D is already initialized");
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

impl InputAction for Goblin {
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
			match self.state.face_direction {
				FaceDirection::Left => self.left_attack(),
				FaceDirection::Right => self.right_attack(),
			}
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
				self.transition_to_walk();
			} else {
				self.state.action = Action::Idle;
				self.transition_to_idle();
			}

			self.base_mut().set_velocity(velocity);
			self.base_mut().move_and_slide();
		}
	}
}