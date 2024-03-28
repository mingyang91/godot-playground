use godot::engine::{AnimationNodeStateMachinePlayback, AnimationTree, CharacterBody2D, ICharacterBody2D, Label};
use godot::prelude::*;

use crate::characters::common::{Action, AttackCoolDown};

#[derive(Debug)]
struct State {
	hp: i32,
	attack_cool_down: AttackCoolDown,
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Warrior {
	#[var]
	gravity: real,
	#[var]
	#[export]
	action: Action,
	speed: real,
	state: State,
	base: Base<CharacterBody2D>,
}


#[godot_api]
impl Warrior {
	#[func]
	fn on_animation_finished(&mut self, name: GString) {
		if name == Action::Attack.to_godot() {
			self.action = Action::Idle
		}
	}

	fn set_direction(&mut self, dir: Vector2) {
		let mut animation_tree = self.base().get_node_as::<AnimationTree>("AnimationTree");
		animation_tree.set("parameters/Attack/blend_position".into(), dir.to_variant());
		if dir.length() > 0.0 {
			animation_tree.set("parameters/Idle/blend_position".into(), dir.to_variant());
			animation_tree.set("parameters/Walk/blend_position".into(), dir.to_variant());
		}
	}
}

#[godot_api]
impl ICharacterBody2D for Warrior {
	fn init(base: Base<CharacterBody2D>) -> Self {
		Warrior {
			gravity: 100 as real,
			speed: 100 as real,
			action: Action::Idle,
			state: State {
				hp: 100,
				attack_cool_down: AttackCoolDown::new(1.0),
			},
			base,
		}
	}

	fn ready(&mut self) {}

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
		match self.action {
			Action::Attack => false,
			Action::Dead => false,
			_ => true
		}
	}

	fn attack_pressed(&mut self) {
		if self.state.attack_cool_down.ready() {
			self.state.attack_cool_down.reset();
			let mut animation_tree = self.base().get_node_as::<AnimationTree>("AnimationTree");
			let mut state_machine: Gd<AnimationNodeStateMachinePlayback> = animation_tree.get("parameters/playback".into()).to();
			state_machine.travel(Action::Attack.to_godot().into());
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

			if velocity.length() > 0.0 {
				velocity = velocity.normalized() * self.speed;
				self.action = Action::Walk;
			} else {
				self.action = Action::Idle;
			}

			self.base_mut().set_velocity(velocity);
			self.set_direction(velocity);
			self.base_mut().move_and_slide();
		}
	}
}