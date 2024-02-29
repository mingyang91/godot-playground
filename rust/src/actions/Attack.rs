trait Attacker {
    fn attack(&self, target: &mut dyn CanBeAttack, damage: i64) -> bool;
}

trait CanBeAttack {
    fn take_damage(&mut self, damage: i64) -> bool;
}
