use std::ops::Add;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Ability {
    pub(crate) strength: u8,
    pub(crate) dexterity: u8,
    pub(crate) constitution: u8,
    pub(crate) intelligence: u8,
    pub(crate) wisdom: u8,
    pub(crate) charisma: u8,
    pub(crate) hit_points: u32,
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
