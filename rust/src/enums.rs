use std::mem::size_of;
use bitflags::{bitflags, Flags};

#[derive(Debug)]
pub enum AbilityType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl From<AbilityBits> for AbilityType {
    fn from(ability: AbilityBits) -> Self {
        match ability {
            AbilityBits::STRENGTH => AbilityType::Strength,
            AbilityBits::DEXTERITY => AbilityType::Dexterity,
            AbilityBits::CONSTITUTION => AbilityType::Constitution,
            AbilityBits::INTELLIGENCE => AbilityType::Intelligence,
            AbilityBits::WISDOM => AbilityType::Wisdom,
            AbilityBits::CHARISMA => AbilityType::Charisma,
            _ => unreachable!("Invalid ability bits"),
        }
    }
}

impl From<AbilityType> for AbilityBits {
    fn from(ability: AbilityType) -> Self {
        match ability {
            AbilityType::Strength => AbilityBits::STRENGTH,
            AbilityType::Dexterity => AbilityBits::DEXTERITY,
            AbilityType::Constitution => AbilityBits::CONSTITUTION,
            AbilityType::Intelligence => AbilityBits::INTELLIGENCE,
            AbilityType::Wisdom => AbilityBits::WISDOM,
            AbilityType::Charisma => AbilityBits::CHARISMA,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AbilityBits: u8 {
        const STRENGTH = 0b00000001;
        const DEXTERITY = 0b00000010;
        const CONSTITUTION = 0b00000100;
        const INTELLIGENCE = 0b00001000;
        const WISDOM = 0b00010000;
        const CHARISMA = 0b00100000;
    }
}

pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}

impl From<ArmorBits> for ArmorType {
    fn from(armor: ArmorBits) -> Self {
        match armor {
            ArmorBits::LIGHT => ArmorType::Light,
            ArmorBits::MEDIUM => ArmorType::Medium,
            ArmorBits::HEAVY => ArmorType::Heavy,
            _ => unreachable!("Invalid armor bits"),
        }
    }
}

impl From<ArmorType> for ArmorBits {
    fn from(armor: ArmorType) -> Self {
        match armor {
            ArmorType::Light => ArmorBits::LIGHT,
            ArmorType::Medium => ArmorBits::MEDIUM,
            ArmorType::Heavy => ArmorBits::HEAVY,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ArmorBits: u8 {
        const LIGHT = 0b00000001;
        const MEDIUM = 0b00000010;
        const HEAVY = 0b00000100;
    }
}

pub enum WeaponType {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WeaponBits: u8 {
        const SIMPLE_MELEE = 0b00000001;
        const SIMPLE_RANGED = 0b00000010;
        const MARTIAL_MELEE = 0b00000100;
        const MARTIAL_RANGED = 0b00001000;
    }
}

impl From<WeaponBits> for WeaponType {
    fn from(weapon: WeaponBits) -> Self {
        match weapon {
            WeaponBits::SIMPLE_MELEE => WeaponType::SimpleMelee,
            WeaponBits::SIMPLE_RANGED => WeaponType::SimpleRanged,
            WeaponBits::MARTIAL_MELEE => WeaponType::MartialMelee,
            WeaponBits::MARTIAL_RANGED => WeaponType::MartialRanged,
            _ => unreachable!("Invalid weapon bits"),
        }
    }
}

impl From<WeaponType> for WeaponBits {
    fn from(weapon: WeaponType) -> Self {
        match weapon {
            WeaponType::SimpleMelee => WeaponBits::SIMPLE_MELEE,
            WeaponType::SimpleRanged => WeaponBits::SIMPLE_RANGED,
            WeaponType::MartialMelee => WeaponBits::MARTIAL_MELEE,
            WeaponType::MartialRanged => WeaponBits::MARTIAL_RANGED,
        }
    }
}

pub enum ShieldType {
    Buckler,
    Heater,
    Kite,
    Tower,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShieldBits: u8 {
        const BUCKLER = 0b00000001;
        const HEATER = 0b00000010;
        const KITE = 0b00000100;
        const TOWER = 0b00001000;
    }
}

impl From<ShieldBits> for ShieldType {
    fn from(value: ShieldBits) -> Self {
        match value {
            ShieldBits::BUCKLER => ShieldType::Buckler,
            ShieldBits::HEATER => ShieldType::Heater,
            ShieldBits::KITE => ShieldType::Kite,
            ShieldBits::TOWER => ShieldType::Tower,
            _ => unreachable!("Invalid shield bits"),
        }
    }
}

impl From<ShieldType> for ShieldBits {
    fn from(value: ShieldType) -> Self {
        match value {
            ShieldType::Buckler => ShieldBits::BUCKLER,
            ShieldType::Heater => ShieldBits::HEATER,
            ShieldType::Kite => ShieldBits::KITE,
            ShieldType::Tower => ShieldBits::TOWER,
        }
    }
}

pub enum SkillType {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SkillBits: u32 {
        const ACROBATICS      = 0b00000000000000000000000000000001;
        const ANIMAL_HANDLING = 0b00000000000000000000000000000010;
        const ARCANA          = 0b00000000000000000000000000000100;
        const ATHLETICS       = 0b00000000000000000000000000001000;
        const DECEPTION       = 0b00000000000000000000000000010000;
        const HISTORY         = 0b00000000000000000000000000100000;
        const INSIGHT         = 0b00000000000000000000000001000000;
        const INTIMIDATION    = 0b00000000000000000000000010000000;
        const INVESTIGATION   = 0b00000000000000000000000100000000;
        const MEDICINE        = 0b00000000000000000000001000000000;
        const NATURE          = 0b00000000000000000000010000000000;
        const PERCEPTION      = 0b00000000000000000000100000000000;
        const PERFORMANCE     = 0b00000000000000000001000000000000;
        const PERSUASION      = 0b00000000000000000010000000000000;
        const RELIGION        = 0b00000000000000000100000000000000;
        const SLEIGHT_OF_HAND = 0b00000000000000001000000000000000;
        const STEALTH         = 0b00000000000000010000000000000000;
        const SURVIVAL        = 0b00000000000000100000000000000000;
    }
}

impl From<SkillBits> for SkillType {
    fn from(value: SkillBits) -> Self {
        match value {
            SkillBits::ACROBATICS => SkillType::Acrobatics,
            SkillBits::ANIMAL_HANDLING => SkillType::AnimalHandling,
            SkillBits::ARCANA => SkillType::Arcana,
            SkillBits::ATHLETICS => SkillType::Athletics,
            SkillBits::DECEPTION => SkillType::Deception,
            SkillBits::HISTORY => SkillType::History,
            SkillBits::INSIGHT => SkillType::Insight,
            SkillBits::INTIMIDATION => SkillType::Intimidation,
            SkillBits::INVESTIGATION => SkillType::Investigation,
            SkillBits::MEDICINE => SkillType::Medicine,
            SkillBits::NATURE => SkillType::Nature,
            SkillBits::PERCEPTION => SkillType::Perception,
            SkillBits::PERFORMANCE => SkillType::Performance,
            SkillBits::PERSUASION => SkillType::Persuasion,
            SkillBits::RELIGION => SkillType::Religion,
            SkillBits::SLEIGHT_OF_HAND => SkillType::SleightOfHand,
            SkillBits::STEALTH => SkillType::Stealth,
            SkillBits::SURVIVAL => SkillType::Survival,
            _ => unreachable!("Invalid skill bits"),
        }
    }
}

impl From<SkillType> for SkillBits {
    fn from(value: SkillType) -> Self {
        match value {
            SkillType::Acrobatics => SkillBits::ACROBATICS,
            SkillType::AnimalHandling => SkillBits::ANIMAL_HANDLING,
            SkillType::Arcana => SkillBits::ARCANA,
            SkillType::Athletics => SkillBits::ATHLETICS,
            SkillType::Deception => SkillBits::DECEPTION,
            SkillType::History => SkillBits::HISTORY,
            SkillType::Insight => SkillBits::INSIGHT,
            SkillType::Intimidation => SkillBits::INTIMIDATION,
            SkillType::Investigation => SkillBits::INVESTIGATION,
            SkillType::Medicine => SkillBits::MEDICINE,
            SkillType::Nature => SkillBits::NATURE,
            SkillType::Perception => SkillBits::PERCEPTION,
            SkillType::Performance => SkillBits::PERFORMANCE,
            SkillType::Persuasion => SkillBits::PERSUASION,
            SkillType::Religion => SkillBits::RELIGION,
            SkillType::SleightOfHand => SkillBits::SLEIGHT_OF_HAND,
            SkillType::Stealth => SkillBits::STEALTH,
            SkillType::Survival => SkillBits::SURVIVAL,
        }
    }
}


pub enum ClassType {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClassBits: u16 {
        const BARBARIAN = 0b0000000000000001;
        const BARD      = 0b0000000000000010;
        const CLERIC    = 0b0000000000000100;
        const DRUID     = 0b0000000000001000;
        const FIGHTER   = 0b0000000000010000;
        const MONK      = 0b0000000000100000;
        const PALADIN   = 0b0000000001000000;
        const RANGER    = 0b0000000010000000;
        const ROGUE     = 0b0000000100000000;
        const SORCERER  = 0b0000001000000000;
        const WARLOCK   = 0b0000010000000000;
        const WIZARD    = 0b0000100000000000;
    }
}

impl From<ClassBits> for ClassType {
    fn from(value: ClassBits) -> Self {
        match value {
            ClassBits::BARBARIAN => ClassType::Barbarian,
            ClassBits::BARD => ClassType::Bard,
            ClassBits::CLERIC => ClassType::Cleric,
            ClassBits::DRUID => ClassType::Druid,
            ClassBits::FIGHTER => ClassType::Fighter,
            ClassBits::MONK => ClassType::Monk,
            ClassBits::PALADIN => ClassType::Paladin,
            ClassBits::RANGER => ClassType::Ranger,
            ClassBits::ROGUE => ClassType::Rogue,
            ClassBits::SORCERER => ClassType::Sorcerer,
            ClassBits::WARLOCK => ClassType::Warlock,
            ClassBits::WIZARD => ClassType::Wizard,
            _ => unreachable!("Invalid class bits"),
        }
    }
}

impl From<ClassType> for ClassBits {
    fn from(value: ClassType) -> Self {
        match value {
            ClassType::Barbarian => ClassBits::BARBARIAN,
            ClassType::Bard => ClassBits::BARD,
            ClassType::Cleric => ClassBits::CLERIC,
            ClassType::Druid => ClassBits::DRUID,
            ClassType::Fighter => ClassBits::FIGHTER,
            ClassType::Monk => ClassBits::MONK,
            ClassType::Paladin => ClassBits::PALADIN,
            ClassType::Ranger => ClassBits::RANGER,
            ClassType::Rogue => ClassBits::ROGUE,
            ClassType::Sorcerer => ClassBits::SORCERER,
            ClassType::Warlock => ClassBits::WARLOCK,
            ClassType::Wizard => ClassBits::WIZARD,
        }
    }
}

pub enum Proficiency {
    Armor(ArmorType),
    Weapon(WeaponType),
    Shield(ShieldType),
    SavingThrows(AbilityType),
    Skill(SkillType)
}

/**
* Proficiencies are a bitfield of the various proficiencies a character has.
*
* The bitfield is broken down into the following sections:
*
* 0-8: Armor
* 9-16: Weapon
* 17-24: Shield
* 25-31: Saving Throws
* 32-63: Skills
*/
type ps = u128;

const ARMOR_START: ps = 0;
const WEAPON_START: ps = size_of::<<ArmorBits as Flags>::Bits>() as ps;
const SHIELD_START: ps = size_of::<<WeaponBits as Flags>::Bits>() as ps + WEAPON_START;
const SAVING_THROW_START: ps = size_of::<<ShieldBits as Flags>::Bits>() as ps + SHIELD_START;
const SKILL_START: ps = size_of::<<AbilityBits as Flags>::Bits>() as ps + SAVING_THROW_START;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Proficiencies: u128 {
        const LIGHT_ARMOR       = (ArmorBits::LIGHT.bits() as ps) << ARMOR_START;
        const MEDIUM_ARMOR      = (ArmorBits::MEDIUM.bits() as ps) << ARMOR_START;
        const HEAVY_ARMOR       = (ArmorBits::HEAVY.bits() as ps) << ARMOR_START;
        const SIMPLE_MELEE      = (WeaponBits::SIMPLE_MELEE.bits() as ps) << WEAPON_START;
        const SIMPLE_RANGED     = (WeaponBits::SIMPLE_RANGED.bits() as ps) << WEAPON_START;
        const MARTIAL_MELEE     = (WeaponBits::MARTIAL_MELEE.bits() as ps) << WEAPON_START;
        const MARTIAL_RANGED    = (WeaponBits::MARTIAL_RANGED.bits() as ps) << WEAPON_START;
        const BUCKLER           = (ShieldBits::BUCKLER.bits() as ps) << SHIELD_START;
        const HEATER            = (ShieldBits::HEATER.bits() as ps) << SHIELD_START;
        const KITE              = (ShieldBits::KITE.bits() as ps) << SHIELD_START;
        const TOWER             = (ShieldBits::TOWER.bits() as ps) << SHIELD_START;
        const STRENGTH_SAVE     = (AbilityBits::STRENGTH.bits() as ps) << SAVING_THROW_START;
        const DEXTERITY_SAVE    = (AbilityBits::DEXTERITY.bits() as ps) << SAVING_THROW_START;
        const CONSTITUTION_SAVE = (AbilityBits::CONSTITUTION.bits() as ps) << SAVING_THROW_START;
        const INTELLIGENCE_SAVE = (AbilityBits::INTELLIGENCE.bits() as ps) << SAVING_THROW_START;
        const WISDOM_SAVE       = (AbilityBits::WISDOM.bits() as ps) << SAVING_THROW_START;
        const CHARISMA_SAVE     = (AbilityBits::CHARISMA.bits() as ps) << SAVING_THROW_START;
        const ACROBATICS        = (SkillBits::ACROBATICS.bits() as ps) << SKILL_START;
        const ANIMAL_HANDLING   = (SkillBits::ANIMAL_HANDLING.bits() as ps) << SKILL_START;
        const ARCANA            = (SkillBits::ARCANA.bits() as ps) << SKILL_START;
        const ATHLETICS         = (SkillBits::ATHLETICS.bits() as ps) << SKILL_START;
        const DECEPTION         = (SkillBits::DECEPTION.bits() as ps) << SKILL_START;
        const HISTORY           = (SkillBits::HISTORY.bits() as ps) << SKILL_START;
        const INSIGHT           = (SkillBits::INSIGHT.bits() as ps) << SKILL_START;
        const INTIMIDATION      = (SkillBits::INTIMIDATION.bits() as ps) << SKILL_START;
        const INVESTIGATION     = (SkillBits::INVESTIGATION.bits() as ps) << SKILL_START;
        const MEDICINE          = (SkillBits::MEDICINE.bits() as ps) << SKILL_START;
        const NATURE            = (SkillBits::NATURE.bits() as ps) << SKILL_START;
        const PERCEPTION        = (SkillBits::PERCEPTION.bits() as ps) << SKILL_START;
        const PERFORMANCE       = (SkillBits::PERFORMANCE.bits() as ps) << SKILL_START;
        const PERSUASION        = (SkillBits::PERSUASION.bits() as ps) << SKILL_START;
        const RELIGION          = (SkillBits::RELIGION.bits() as ps) << SKILL_START;
        const SLEIGHT_OF_HAND   = (SkillBits::SLEIGHT_OF_HAND.bits() as ps) << SKILL_START;
        const STEALTH           = (SkillBits::STEALTH.bits() as ps) << SKILL_START;
        const SURVIVAL          = (SkillBits::SURVIVAL.bits() as ps) << SKILL_START;
    }
}

impl Proficiencies {
    pub fn new() -> Self {
        Proficiencies::empty()
    }

    #[inline]
    pub fn from_proficiencies(proficiencies: Vec<Proficiency>) -> Self {
        let mut p = Proficiencies::empty();
        for proficiency in proficiencies {
            p.add_proficiency(proficiency);
        }
        p
    }

    #[inline]
    pub fn add_proficiency(&mut self, proficiency: Proficiency) {
        match proficiency {
            Proficiency::Armor(armor) => self.add_armor(armor),
            Proficiency::Weapon(weapon) => self.add_weapon(weapon),
            Proficiency::Shield(shield) => self.add_shield(shield),
            Proficiency::SavingThrows(ability) => self.add_saving_throw(ability),
            Proficiency::Skill(skill) => self.add_skill(skill),
        }
    }

    #[inline]
    pub fn add_armor(&mut self, armor: ArmorType) {
        let ab = ArmorBits::from(armor);
        self.insert(Proficiencies::from_bits_truncate((ab.bits() as ps) << ARMOR_START));
    }

    #[inline]
    pub fn add_weapon(&mut self, weapon: WeaponType) {
        let wb = WeaponBits::from(weapon);
        self.insert(Proficiencies::from_bits_truncate((wb.bits() as ps) << WEAPON_START));
    }

    #[inline]
    pub fn add_shield(&mut self, shield: ShieldType) {
        let sb = ShieldBits::from(shield);
        self.insert(Proficiencies::from_bits_truncate((sb.bits() as ps) << SHIELD_START));
    }

    #[inline]
    pub fn add_saving_throw(&mut self, ability: AbilityType) {
        let ab = AbilityBits::from(ability);
        self.insert(Proficiencies::from_bits_truncate((ab.bits() as ps) << SAVING_THROW_START));
    }

    #[inline]
    pub fn add_skill(&mut self, skill: SkillType) {
        let sb = SkillBits::from(skill);
        self.insert(Proficiencies::from_bits_truncate((sb.bits() as ps) << SKILL_START));
    }

    #[inline]
    pub fn get_proficiency_list(&self) -> Vec<Proficiency> {
        let mut list = Vec::new();
        list.extend(self.get_armor_list().into_iter().map(|a| Proficiency::Armor(a)));
        list.extend(self.get_weapon_list().into_iter().map(|w| Proficiency::Weapon(w)));
        list.extend(self.get_shield_list().into_iter().map(|s| Proficiency::Shield(s)));
        list.extend(self.get_saving_throw_list().into_iter().map(|a| Proficiency::SavingThrows(a)));
        list.extend(self.get_skill_list().into_iter().map(|s| Proficiency::Skill(s)));
        list
    }

    #[inline]
    pub fn get_armor_bits(&self) -> ArmorBits {
        let armor = (self.bits() >> ARMOR_START) as u8;
        ArmorBits::from_bits(armor).expect("Invalid armor bits")
    }

    #[inline]
    pub fn get_armor_list(&self) -> Vec<ArmorType> {
        let armor_bits = self.get_armor_bits();
        return ArmorBits::FLAGS
            .iter()
            .filter(|&f| armor_bits.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_weapon_bits(&self) -> WeaponBits {
        let weapon = (self.bits() >> WEAPON_START) as u8;
        WeaponBits::from_bits(weapon).expect("Invalid weapon bits")
    }

    #[inline]
    pub fn get_weapon_list(&self) -> Vec<WeaponType> {
        let weapon_bits = self.get_weapon_bits();
        return WeaponBits::FLAGS
            .iter()
            .filter(|&f| weapon_bits.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_shield_bits(&self) -> ShieldBits {
        let shield = (self.bits() >> SHIELD_START) as u8;
        ShieldBits::from_bits(shield).expect("Invalid shield bits")
    }

    #[inline]
    pub fn get_shield_list(&self) -> Vec<ShieldType> {
        let shield_bits = self.get_shield_bits();
        return ShieldBits::FLAGS
            .iter()
            .filter(|&f| shield_bits.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_saving_throw_bits(&self) -> AbilityBits {
        let saving_throw = (self.bits() >> SAVING_THROW_START) as u8;
        AbilityBits::from_bits(saving_throw).expect("Invalid saving throw bits")
    }

    #[inline]
    pub fn get_saving_throw_list(&self) -> Vec<AbilityType> {
        let saving_throw_bits = self.get_saving_throw_bits();
        return AbilityBits::FLAGS
            .iter()
            .filter(|&f| saving_throw_bits.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_skill_bits(&self) -> SkillBits {
        let skill = (self.bits() >> SKILL_START) as u32;
        SkillBits::from_bits(skill).expect("Invalid skill bits")
    }

    #[inline]
    pub fn get_skill_list(&self) -> Vec<SkillType> {
        let skill_bits = self.get_skill_bits();
        return SkillBits::FLAGS
            .iter()
            .filter(|&f| skill_bits.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }
}