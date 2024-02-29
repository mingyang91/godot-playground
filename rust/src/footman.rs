use std::fmt::Display;
use crate::alignment::Alignment;

pub struct Ability {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
    hit_points: u32,
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ability: str: {}, dex: {}, con: {}, int: {}, wis: {}, cha: {}, hp: {}",
               self.strength, self.dexterity, self.constitution, self.intelligence,
               self.wisdom, self.charisma, self.hit_points)
    }
}

trait CharacterClass {
    fn name(&self) -> String;
    fn hit_dice(&self) -> u8;
    fn proficiencies(&self) -> Vec<String>;
    fn saving_throws(&self) -> Vec<String>;
}

trait CharacterRace {
    fn name(&self) -> String;
    fn size(&self) -> String;
    fn speed(&self) -> u8;
    fn languages(&self) -> Vec<String>;
    fn traits(&self) -> Vec<String>;
}

trait Character {
    type Class: CharacterClass;
    type Race: CharacterRace;
    fn ability(&self) -> Ability;
    fn name(&self) -> String;
    fn level(&self) -> u32;
    fn health(&self) -> u32;
    fn is_alive(&self) -> bool {
        self.health() > 0
    }

    fn armor_class(&self) -> u8;
    fn alignment(&self) -> Alignment;
    fn class(&self) -> Self::Class;
    fn race(&self) -> Self::Race;
    // fn skills(&self) -> Vec<Skill>;
    // fn equipment(&self) -> Vec<Equipment>;
    // fn spells(&self) -> Vec<Spell>;
}
trait Weapon {
    fn attack(&self);
}

trait Shield {
    fn defend(&self);
}

trait Warrior: Character {
    type Weapon: Weapon;
    type Shield: Shield;
    fn attack(&self);
}