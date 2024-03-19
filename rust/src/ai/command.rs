use godot::prelude::*;

pub enum Command {
    Move(Move),
    Attack(Attack)
}

pub enum Move {
    Target(Vector2),
    Direction(Vector2),
}

pub enum AttackType {
    Melee,
    Ranged,
    Skill(String),
}

pub enum AttackTarget {
    Target(Gd<Node2D>),
    Direction(Vector2),
}

pub struct Attack {
    pub r#type: AttackType,
    pub target: AttackTarget,
}

