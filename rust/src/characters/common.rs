#[derive(Debug)]
pub enum FaceDirection {
	Left,
	Right,
}

#[derive(Debug)]
pub enum Action {
	Idle,
	Walk,
	Attack,
	Die,
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
