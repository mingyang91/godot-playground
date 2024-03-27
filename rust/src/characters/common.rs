use std::fmt::Display;
use godot::engine::global::PropertyHint;
use godot::prelude::{Export, FromGodot, GodotConvert, GString, ToGodot, Var};
use godot::register::property::export_info_functions::export_enum;
use godot::register::property::PropertyHintInfo;

#[derive(Debug)]
pub enum FaceDirection {
	Left,
	Right,
}

impl Display for FaceDirection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			FaceDirection::Left => write!(f, "left"),
			FaceDirection::Right => write!(f, "right"),
		}
	}
}

#[derive(GodotConvert, Debug, Eq, PartialEq)]
#[godot(via = GString)]
pub enum Action {
	Idle,
	Walk,
	Attack,
	Die,
}

impl Var for Action {
	fn get_property(&self) -> Self::Via {
		self.to_godot()
	}

	fn set_property(&mut self, value: Self::Via) {
		*self = Self::from_godot(value)
	}

	fn property_hint() -> PropertyHintInfo {
		PropertyHintInfo {
			hint: PropertyHint::ENUM,
			hint_string: "Idle,Walk,Attack,Die".into(),
		}
	}
}

impl Export for Action {
	fn default_export_info() -> PropertyHintInfo {
		Self::property_hint()
	}
}


impl Display for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Action::Idle => write!(f, "idle"),
			Action::Walk => write!(f, "walk"),
			Action::Attack => write!(f, "attack"),
			Action::Die => write!(f, "die"),
		}
	}
}

#[derive(Debug)]
pub struct AttackCoolDown {
	time: f64,
	elapsed: f64,
}

impl AttackCoolDown {
	pub fn new(time: f64) -> Self {
		AttackCoolDown {
			time,
			elapsed: 0.0,
		}
	}

	pub fn update(&mut self, delta: f64) {
		self.elapsed += delta;
	}

	pub fn reset(&mut self) {
		self.elapsed = 0.0;
	}

	pub fn ready(&self) -> bool {
		self.elapsed >= self.time
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_attack_cool_down() {
		let mut cool_down = AttackCoolDown::new(1.0);
		assert_eq!(cool_down.ready(), true);
		cool_down.update(0.5);
		assert_eq!(cool_down.ready(), false);
		cool_down.update(0.5);
		assert_eq!(cool_down.ready(), true);
		cool_down.update(1.0);
		assert_eq!(cool_down.ready(), true);
	}
}