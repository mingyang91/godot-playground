use std::fmt::Display;
use std::ops::Add;
use crate::alignment::{Alignment, Ethical, Moral};
use crate::enums::{Proficiencies, ShieldType, WeaponType};

#[derive(Debug, Clone)]
pub struct Ability {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
    hit_points: u32,
}

impl Add for Ability {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Ability {
            strength: self.strength + other.strength,
            dexterity: self.dexterity + other.dexterity,
            constitution: self.constitution + other.constitution,
            intelligence: self.intelligence + other.intelligence,
            wisdom: self.wisdom + other.wisdom,
            charisma: self.charisma + other.charisma,
            hit_points: self.hit_points + other.hit_points,
        }
    }
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ability: str: {}, dex: {}, con: {}, int: {}, wis: {}, cha: {}, hp: {}",
               self.strength, self.dexterity, self.constitution, self.intelligence,
               self.wisdom, self.charisma, self.hit_points)
    }
}

#[derive(Debug, Clone)]
struct CharacterClass {
    name: String,
    proficiencies: Proficiencies,
    ability_modifier: Ability
}

#[derive(Debug, Clone)]
struct CharacterRace {
    name: String,
    size: String,
    speed: u8,
    // languages: Vec<String>,
    ability_modifier: Ability,
}

trait Character {
    fn ability(&self) -> &Ability;
    fn name(&self) -> String;
    fn level(&self) -> u32;
    fn health(&self) -> u32;
    fn is_alive(&self) -> bool {
        self.health() > 0
    }

    fn armor_class(&self) -> u8;
    fn alignment(&self) -> Alignment;
    fn class(&self) -> &CharacterClass;
    fn race(&self) -> &CharacterRace;
    // fn skills(&self) -> Vec<Skill>;
    // fn equipment(&self) -> Vec<Equipment>;
    // fn spells(&self) -> Vec<Spell>;
}
trait Weapon {
    fn r#type(&self) -> WeaponType;
    fn attack(&self);
}

trait Shield {
    fn r#type(&self) -> ShieldType;
    fn defend(&self);
}

struct HumanWarrior {
    ability: Ability,
    name: String,
    level: u32,
    health: u32,
    armor_class: u8,
    alignment: Alignment,
    proficiencies: Proficiencies,
}

impl Character for HumanWarrior {

    fn ability(&self) -> &Ability {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }

    fn level(&self) -> u32 {
        todo!()
    }

    fn health(&self) -> u32 {
        todo!()
    }

    fn armor_class(&self) -> u8 {
        todo!()
    }

    fn alignment(&self) -> Alignment {
        todo!()
    }

    fn class(&self) -> &CharacterClass {
        todo!()
    }

    fn race(&self) -> &CharacterRace {
        todo!()
    }
}

impl HumanWarrior {
    fn new(name: String, level: u32, health: u32, armor_class: u8) -> Self {
        let human = CharacterRace {
            name: "Human".to_string(),
            size: "Medium".to_string(),
            speed: 30,
            ability_modifier: Ability {
                strength: 1,
                dexterity: 1,
                constitution: 1,
                intelligence: 1,
                wisdom: 1,
                charisma: 1,
                hit_points: 0,
            }
        };

        let warrior = CharacterClass {
            name: "Warrior".to_string(),
            proficiencies: Proficiencies::new(),
            ability_modifier: Ability {
                strength: 1,
                dexterity: 1,
                constitution: 1,
                intelligence: 1,
                wisdom: 1,
                charisma: 1,
                hit_points: 0,
            }
        };

        let base_ability = Ability {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
            hit_points: 10,
        };
        let ability = base_ability + human.ability_modifier + warrior.ability_modifier;
        HumanWarrior {
            ability,
            name,
            level,
            health,
            armor_class,
            alignment: Alignment::new(
                Moral::Good,
                Ethical::Neutral,
            ),
            proficiencies: Proficiencies::new()
        }
    }
}