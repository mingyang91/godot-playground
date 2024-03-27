use std::cell::OnceCell;

use godot::engine::{AnimationTree, Area2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, Label, NavigationAgent2D, NavigationServer2D, Sprite2D};
use godot::prelude::*;

use crate::characters::common::{Action, AttackCoolDown, FaceDirection};
use crate::dnd::enums::WeaponType;
use crate::interactable::effect::{Effect, Effects};
use crate::interactable::effect::Damage;
use crate::interactable::hit_box::HitBox;
use crate::interactable::navigator::Navigator;
use crate::interactable::sight::SightArea2D;
use crate::tools::weapon::{SimpleMeleeWeapon, Weapon};

#[derive(Debug)]
struct State {
	hp: i32,
    attack_cool_down: AttackCoolDown,
}

#[derive(Debug)]
struct Torch {}

impl Weapon for Torch {
    fn r#type(&self) -> WeaponType {
        WeaponType::SimpleMelee
    }

    fn damage(&self) -> u32 {
        todo!()
    }

    fn range(&self) -> u32 {
        todo!()
    }
}

impl SimpleMeleeWeapon for Torch {}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Goblin {
	#[var]
	gravity: real,
	#[var]
	speed: real,
	#[export]
	action: Action,
	#[export]
	face_direction_name: GString,
	state: State,
	navigator: OnceCell<Navigator>,
    weapon: Torch,
	hit_center_point: Vector2,
	base: Base<CharacterBody2D>,
}


#[godot_api]
impl Goblin {

	#[func]
	fn on_attack_end(&mut self) {
		self.action = Action::Idle;
	}

	#[func]
	fn on_animation_changed(&mut self, old_name: Variant, _new_name: Variant) {
		tracing::debug!("animation changed: {:?} -> {:?}", old_name, _new_name);
	}

	#[func]
	fn on_enemy_entered(&mut self, body: Gd<Area2D>) {
		let Some(owner) = body.get_owner() else {
			return
		};
		tracing::debug!("enemy entered: {:?}", owner);
		if !self.get_navigator_mut().is_following() {
			self.get_navigator_mut().follow(owner.cast());
		}
		// TODO: send to AI to decide what to do
	}

	#[func]
	fn on_enemy_exited(&mut self, body: Gd<Area2D>) {
		let Some(owner) = body.get_owner() else {
			return
		};
		tracing::debug!("enemy exited: {:?}", owner);
		if self.get_navigator_mut().get_target() == Some(owner.cast()) {
			self.get_navigator_mut().stop_following();
		}
	}

	fn get_animation_tree(&self) -> Gd<AnimationTree> {
		self.base().get_node_as::<AnimationTree>("AnimationTree")
	}

	fn get_navigator(&self) -> &Navigator {
		self.navigator.get().expect("Navigator is not initialized")
	}

	fn get_navigator_mut(&mut self) -> &mut Navigator {
		self.navigator.get_mut().expect("Navigator is not initialized")
	}

	fn turn_left(&mut self) {
		self.face_direction_name = FaceDirection::Left.to_string().into();
	}

	fn turn_right(&mut self) {
		self.face_direction_name = FaceDirection::Right.to_string().into();
	}

	fn transition_to_idle(&mut self) {
		self.action = Action::Idle;
	}

	fn transition_to_walk(&mut self) {
		self.action = Action::Walk;
	}

	fn attack(&mut self) {
		self.action = Action::Attack;
	}

	fn up_attack(&mut self) {
		self.action = Action::Attack;
	}

	fn down_attack(&mut self) {
		self.action = Action::Attack;
	}
}

#[godot_api]
impl ICharacterBody2D for Goblin {
	fn init(base: Base<CharacterBody2D>) -> Self {
		Goblin {
			gravity: 100 as real,
			speed: 100 as real,
			action: Action::Idle,
			face_direction_name: FaceDirection::Right.to_string().into(),
			state: State {
				hp: 100,
				attack_cool_down: AttackCoolDown::new(1.0),
			},
            weapon: Torch {},
			navigator: OnceCell::new(),
			hit_center_point: Vector2::ZERO,
			base,
		}
	}

	fn ready(&mut self) {
		self.base().get_world_2d();
		let navigation_agent = self.base_mut()
			.get_node_as::<NavigationAgent2D>("NavigationAgent2D");
		self.navigator.set(Navigator::new(navigation_agent))
			.expect("NavigationAgent2D is already initialized");

        let mut hit_box = self.base_mut().get_node_as::<HitBox>("Sprite2D/HitBox");
        let effects = Effects::new(vec![Effect::Damage(Damage { amount: 10 })]);
        hit_box.bind_mut().set_effects(effects);
	}

	fn process(&mut self, delta: f64) {
		self.state.attack_cool_down.update(delta);
		// self.process_input()
		let sight = self.base_mut().get_node_as::<SightArea2D>("SightArea2D");
		if !self.get_navigator().is_following() {
			for area in sight.get_overlapping_areas().iter_shared() {
				let Some(owner) = area.get_owner() else {
					tracing::debug!("Dangling node: {:?}", area);
					continue
				};
				// tracing::debug!("insight: {:?}", owner);
				self.get_navigator_mut().follow(owner.cast());
				break
			}
		} else if !sight.has_overlapping_areas() {
			self.get_navigator_mut().stop_following();
			// tracing::debug!("lost sight");
		}

		if !self.get_navigator().is_target_reached() {
			let self_pos = self.base().get_global_position();
			let next_pos = self.get_navigator_mut().get_next_position();
			let direction = next_pos - self_pos;
			if direction.length() > 0.0 {
				let speed = self.speed;
				let mut animation_tree = self.base().get_node_as::<AnimationTree>("AnimationTree");
				animation_tree.set("parameters/walk/blend_position".into(), direction.x.to_variant());
				animation_tree.set("parameters/idle/blend_position".into(), direction.x.to_variant());
				animation_tree.set("parameters/attack/blend_position".into(), direction.to_variant());

				self.transition_to_walk();
				self.base_mut().set_velocity(direction.normalized() * speed);
				self.base_mut().move_and_slide();
			} else {
				self.transition_to_idle();
				self.base_mut().set_velocity(Vector2::ZERO);
			}
		} else {
			self.transition_to_idle();
			self.base_mut().set_velocity(Vector2::ZERO);
		}

		let mut debug = self.base().get_node_as::<Label>("Debug");
		debug.set_text(format!("follow: {:?}, state: {:?}", self.navigator, self.state).into());
	}
}

trait InputAction {
	fn is_accept_input(&self) -> bool;
	fn attack_pressed(&mut self);

	fn process_input(&mut self);
}

impl InputAction for Goblin {
	fn is_accept_input(&self) -> bool {
		match self.action {
			Action::Attack => false,
			Action::Die => false,
			_ => true
		}
	}

	fn attack_pressed(&mut self) {
		if self.state.attack_cool_down.ready() {
			self.state.attack_cool_down.reset();
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
				self.transition_to_walk();
			} else {
				self.transition_to_idle();
			}

			self.base_mut().set_velocity(velocity);
			self.base_mut().move_and_slide();
		}
	}
}
