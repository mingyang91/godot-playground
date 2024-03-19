use godot::prelude::*;
use crate::dnd::ability::Ability;

// TODO
pub struct Buff {
    pub name: String
}

pub struct Attributes {
    pub ability: Ability,
    pub hit_points: u32,
    pub mana_points: u32,
    pub buffs: Vec<Buff>,
}

pub struct State {
    pub position: Vector2,
    pub attributes: Attributes,
}

pub struct Environment {
    pub time: f64,
    pub delta: f64,
    pub characters_in_attack_range: Vec<Gd<Node2D>>,
    pub characters_in_sight: Vec<Gd<Node2D>>,
}

pub trait Strategy {
    fn evaluate(&self, state: &State, env: &Environment) -> Command;
}

pub struct Aggroed {
    pub target: Gd<Node2D>,
    pub position: Vector2,
    pub attack_range: real,
}

impl Strategy for Aggroed {
    fn evaluate(&self, _state: &State, _env: &Environment) -> Command {
        if self.target.get_position().distance_to(self.position) < self.attack_range {
            Command::Attack(Attack::Direction {
                direction: self.target.get_position() - self.position,
            })
        } else {
            Command::Move(self.target.get_position())
        }
    }
}

pub struct Sentry {
    pub stay_position: Vector2,
    pub follow_range: real,
}

impl Strategy for Sentry {
    fn evaluate(&self, state: &State, env: &Environment) -> Command {
        // attack if in range
        if let Some(closest_character) = env.characters_in_attack_range.first() {
            return Command::Attack(Attack::LockOn {
                target: closest_character.clone(),
            })
        }

        // move back to position if out of range
        if state.position.distance_to(self.stay_position) > self.follow_range {
            return Command::Move(self.stay_position)
        }

        // move to character if in sight
        if let Some(visible_character) = env.characters_in_sight.first() {
            return Command::Move(visible_character.get_position())
        }

        Command::ContinueLast
    }
}

pub struct Intelligence {
    pub strategies: Vec<Box<dyn Strategy>>
}

pub trait Behavior<I, A> {
    fn update(&mut self, info: I, delta: f64);
    fn action(&self) -> A;
}

pub enum Command {
    ContinueLast,
    Move(Vector2),
    Attack(Attack),
    Skill(Skill),
    Stop,
    Hold,
}

pub enum Attack {
    LockOn {
        target: Gd<Node2D>,
    },
    Direction {
        direction: Vector2,
    },
}

pub enum AttackType {
    Melee,
    Ranged,
}

// Offensive skills
#[derive(Debug)]
enum OffensiveSkill {
    DirectDamage { damage: i32 },
    DamageOverTime { damage: i32, duration: i32, tick_rate: real },
    AreaOfEffect { damage: i32, radius: real },
    PiercingAttack { damage: i32, armor_penetration: i32 },
    CriticalStrike { damage: i32, crit_chance: real },
}

// Defensive skills
#[derive(Debug)]
enum DefensiveSkill {
    Shielding { shield_strength: i32 },
    Healing { heal_amount: i32 },
    DamageAbsorption { absorb_limit: i32 },
    Evasion { evasion_chance: real },
    CrowdControlResistance { duration_reduction: real },
}

// Utility skills
#[derive(Debug)]
enum UtilitySkill {
    Buffing { ability: Ability, duration: i32 },
    Debuffing { ability: Ability, duration: i32 },
    Summoning { summon_id: String },
    Teleportation { distance: real },
    TrapSetting { trap_id: String },
}

// Crowd control skills
#[derive(Debug)]
enum CrowdControlSkill {
    Stun { duration: i32 },
    Slow { amount: real, duration: i32 },
    Knockback { distance: real },
    Silence { duration: i32 },
    Fear { duration: i32 },
    Charm { duration: i32 },
}

// Elemental/Magic skills
#[derive(Debug)]
enum ElementalSkill {
    Fire { damage: i32, area: real },
    Ice { damage: i32, slow_effect: real },
    Lightning { damage: i32, chain_targets: i32 },
    Earth { damage: i32, entangle_duration: i32 },
    Dark { damage: i32, drain_health: bool },
    Arcane { damage: i32, warp_distance: real },
}

// General Skill enum that encompasses all types of skills
#[derive(Debug)]
enum SkillType {
    Offensive(OffensiveSkill),
    Defensive(DefensiveSkill),
    Utility(UtilitySkill),
    CrowdControl(CrowdControlSkill),
    Elemental(ElementalSkill),
}

#[derive(Debug)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub skill_types: Vec<SkillType>,
    pub cooldown: f64,
    pub elapsed: f64,
}