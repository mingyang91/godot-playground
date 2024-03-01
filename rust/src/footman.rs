use std::fmt::Display;
use crate::ability::Ability;
use crate::alignment::{Alignment, Ethical, Moral};
use crate::enums::{Proficiencies, ShieldType, WeaponType};

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

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod test {
    use crate::enums::{AbilityType, ArmorType, ShieldType, SkillType, WeaponType};

    #[test]
    fn test_human_warrior() {
        let mut human_warrior = super::HumanWarrior::new("Bob".to_string(), 1, 10, 10);
        assert_eq!(human_warrior.name, "Bob");
        assert_eq!(human_warrior.level, 1);
        assert_eq!(human_warrior.health, 10);
        assert_eq!(human_warrior.armor_class, 10);
        assert_eq!(human_warrior.alignment, super::Alignment::new(
            super::Moral::Good,
            super::Ethical::Neutral,
        ));

        println!("{:?}", human_warrior);
        human_warrior.proficiencies.add_armor(ArmorType::Light);
        human_warrior.proficiencies.add_skill(SkillType::Acrobatics);
        human_warrior.proficiencies.add_shield(ShieldType::Buckler);
        human_warrior.proficiencies.add_saving_throw(AbilityType::Dexterity);
        human_warrior.proficiencies.add_saving_throw(AbilityType::Intelligence);
        human_warrior.proficiencies.add_weapon(WeaponType::MartialMelee);
        println!("{:?}", human_warrior.proficiencies.get_proficiency_list());
    }
}