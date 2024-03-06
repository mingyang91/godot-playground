use std::cell::OnceCell;
use godot::engine::{AnimatedSprite2D, AnimationPlayer, CharacterBody2D, ICharacterBody2D, Label, Sprite2D};
use godot::prelude::*;
use crate::characters::common::{Action, AttackCoolDown, FaceDirection};

#[derive(Debug)]
struct State {
	hp: i32,
	action: Action,
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
	animation_player: OnceCell<Gd<AnimationPlayer>>,
	base: Base<CharacterBody2D>,
}


#[godot_api]
impl Goblin {
	#[func]
	fn on_animation_changed(&mut self, old_name: Variant, _new_name: Variant) {
		tracing::info!("animation finished: {:?}", old_name);
		if old_name.to::<String>() == "attack" {
			match self.state.action {
				Action::Attack => self.transition_to_idle(),
				Action::Die => {}
				_ => {}
			}
		}
	}

	fn get_animation_player(&self) -> &Gd<AnimationPlayer> {
		self.animation_player.get().expect("AnimatedSprite2D is not initialized")
	}

	fn get_animation_player_mut(&mut self) -> &mut Gd<AnimationPlayer> {
		self.animation_player.get_mut().expect("AnimatedSprite2D is not initialized")
	}

	fn turn_left(&mut self) {
		self.base_mut()
			.get_node_as::<Sprite2D>("Sprite2D")
			.set_scale(Vector2::new(-1.0, 1.0));
	}

	fn turn_right(&mut self) {
		self.base_mut()
			.get_node_as::<Sprite2D>("Sprite2D")
			.set_scale(Vector2::new(1.0, 1.0));
	}

	fn transition_to_idle(&mut self) {
		self.state.action = Action::Idle;
		self.idle();
	}

	fn transition_to_walk(&mut self) {
		self.state.action = Action::Walk;
		self.walk();
	}

	fn idle(&mut self) {
		self.get_animation_player_mut()
			.play_ex()
			.name("idle".into())
			.done();
	}

	fn walk(&mut self) {
		self.get_animation_player_mut()
			.play_ex()
			.name("walk".into())
			.done();
	}

	fn attack(&mut self) {
		let a = self.get_animation_player_mut()
			.animation_set_next("attack".into(), "idle".into());
		self.get_animation_player_mut()//.play();
			.play_ex()
			.name("attack".into())
			.done();
	}

	fn up_attack(&mut self) {
		self.get_animation_player_mut()
			.play_ex()
			.name("up_attack".into())
			.done();
	}

	fn down_attack(&mut self) {
		self.get_animation_player_mut()
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
				attack_cool_down: AttackCoolDown::new(1.0),
			},
			animation_player: OnceCell::new(),
			base,
		}
	}

	fn ready(&mut self) {
		let mut anime = self.base_mut()
			.get_node_as::<AnimationPlayer>("AnimationPlayer");
		anime.play_ex().name("idle".into()).done();
		self.animation_player.set(anime).expect("AnimationPlayer is already initialized");
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
			self.attack();
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
				self.turn_right();
				velocity += Vector2::RIGHT;
			}
			if input.is_action_pressed("move_left".into()) {
				self.turn_left();
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