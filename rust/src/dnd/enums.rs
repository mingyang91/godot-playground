use bitflags::{bitflags, Flags};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AbilityType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[rustfmt::skip]
impl From<AbilityBits> for AbilityType {
    fn from(ability: AbilityBits) -> Self {
        match ability {
            AbilityBits::STRENGTH     => AbilityType::Strength,
            AbilityBits::DEXTERITY    => AbilityType::Dexterity,
            AbilityBits::CONSTITUTION => AbilityType::Constitution,
            AbilityBits::INTELLIGENCE => AbilityType::Intelligence,
            AbilityBits::WISDOM       => AbilityType::Wisdom,
            AbilityBits::CHARISMA     => AbilityType::Charisma,
            _                         => unreachable!("Invalid ability bits"),
        }
    }
}

#[rustfmt::skip]
impl From<AbilityType> for AbilityBits {
    fn from(ability: AbilityType) -> Self {
        match ability {
            AbilityType::Strength     => AbilityBits::STRENGTH,
            AbilityType::Dexterity    => AbilityBits::DEXTERITY,
            AbilityType::Constitution => AbilityBits::CONSTITUTION,
            AbilityType::Intelligence => AbilityBits::INTELLIGENCE,
            AbilityType::Wisdom       => AbilityBits::WISDOM,
            AbilityType::Charisma     => AbilityBits::CHARISMA,
        }
    }
}

#[rustfmt::skip]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AbilityBits: u8 {
        const STRENGTH     = 0b00000001;
        const DEXTERITY    = 0b00000010;
        const CONSTITUTION = 0b00000100;
        const INTELLIGENCE = 0b00001000;
        const WISDOM       = 0b00010000;
        const CHARISMA     = 0b00100000;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}

#[rustfmt::skip]
impl From<ArmorBits> for ArmorType {
    fn from(armor: ArmorBits) -> Self {
        match armor {
            ArmorBits::LIGHT  => ArmorType::Light,
            ArmorBits::MEDIUM => ArmorType::Medium,
            ArmorBits::HEAVY  => ArmorType::Heavy,
            _                 => unreachable!("Invalid armor bits"),
        }
    }
}

#[rustfmt::skip]
impl From<ArmorType> for ArmorBits {
    fn from(armor: ArmorType) -> Self {
        match armor {
            ArmorType::Light  => ArmorBits::LIGHT,
            ArmorType::Medium => ArmorBits::MEDIUM,
            ArmorType::Heavy  => ArmorBits::HEAVY,
        }
    }
}

#[rustfmt::skip]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ArmorBits: u8 {
        const LIGHT  = 0b00000001;
        const MEDIUM = 0b00000010;
        const HEAVY  = 0b00000100;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponType {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
}

#[rustfmt::skip]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WeaponBits: u8 {
        const SIMPLE_MELEE   = 0b00000001;
        const SIMPLE_RANGED  = 0b00000010;
        const MARTIAL_MELEE  = 0b00000100;
        const MARTIAL_RANGED = 0b00001000;
    }
}

#[rustfmt::skip]
impl From<WeaponBits> for WeaponType {
    fn from(weapon: WeaponBits) -> Self {
        match weapon {
            WeaponBits::SIMPLE_MELEE   => WeaponType::SimpleMelee,
            WeaponBits::SIMPLE_RANGED  => WeaponType::SimpleRanged,
            WeaponBits::MARTIAL_MELEE  => WeaponType::MartialMelee,
            WeaponBits::MARTIAL_RANGED => WeaponType::MartialRanged,
            _                          => unreachable!("Invalid weapon bits"),
        }
    }
}

#[rustfmt::skip]
impl From<WeaponType> for WeaponBits {
    fn from(weapon: WeaponType) -> Self {
        match weapon {
            WeaponType::SimpleMelee   => WeaponBits::SIMPLE_MELEE,
            WeaponType::SimpleRanged  => WeaponBits::SIMPLE_RANGED,
            WeaponType::MartialMelee  => WeaponBits::MARTIAL_MELEE,
            WeaponType::MartialRanged => WeaponBits::MARTIAL_RANGED,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShieldType {
    Buckler,
    Heater,
    Kite,
    Tower,
}

#[rustfmt::skip]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShieldBits: u8 {
        const BUCKLER = 0b00000001;
        const HEATER  = 0b00000010;
        const KITE    = 0b00000100;
        const TOWER   = 0b00001000;
    }
}

#[rustfmt::skip]
impl From<ShieldBits> for ShieldType {
    fn from(value: ShieldBits) -> Self {
        match value {
            ShieldBits::BUCKLER => ShieldType::Buckler,
            ShieldBits::HEATER  => ShieldType::Heater,
            ShieldBits::KITE    => ShieldType::Kite,
            ShieldBits::TOWER   => ShieldType::Tower,
            _                   => unreachable!("Invalid shield bits"),
        }
    }
}

#[rustfmt::skip]
impl From<ShieldType> for ShieldBits {
    fn from(value: ShieldType) -> Self {
        match value {
            ShieldType::Buckler => ShieldBits::BUCKLER,
            ShieldType::Heater  => ShieldBits::HEATER,
            ShieldType::Kite    => ShieldBits::KITE,
            ShieldType::Tower   => ShieldBits::TOWER,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
impl From<SkillBits> for SkillType {
    fn from(value: SkillBits) -> Self {
        match value {
            SkillBits::ACROBATICS      => SkillType::Acrobatics,
            SkillBits::ANIMAL_HANDLING => SkillType::AnimalHandling,
            SkillBits::ARCANA          => SkillType::Arcana,
            SkillBits::ATHLETICS       => SkillType::Athletics,
            SkillBits::DECEPTION       => SkillType::Deception,
            SkillBits::HISTORY         => SkillType::History,
            SkillBits::INSIGHT         => SkillType::Insight,
            SkillBits::INTIMIDATION    => SkillType::Intimidation,
            SkillBits::INVESTIGATION   => SkillType::Investigation,
            SkillBits::MEDICINE        => SkillType::Medicine,
            SkillBits::NATURE          => SkillType::Nature,
            SkillBits::PERCEPTION      => SkillType::Perception,
            SkillBits::PERFORMANCE     => SkillType::Performance,
            SkillBits::PERSUASION      => SkillType::Persuasion,
            SkillBits::RELIGION        => SkillType::Religion,
            SkillBits::SLEIGHT_OF_HAND => SkillType::SleightOfHand,
            SkillBits::STEALTH         => SkillType::Stealth,
            SkillBits::SURVIVAL        => SkillType::Survival,
            _                          => unreachable!("Invalid skill bits"),
        }
    }
}

#[rustfmt::skip]
impl From<SkillType> for SkillBits {
    fn from(value: SkillType) -> Self {
        match value {
            SkillType::Acrobatics     => SkillBits::ACROBATICS,
            SkillType::AnimalHandling => SkillBits::ANIMAL_HANDLING,
            SkillType::Arcana         => SkillBits::ARCANA,
            SkillType::Athletics      => SkillBits::ATHLETICS,
            SkillType::Deception      => SkillBits::DECEPTION,
            SkillType::History        => SkillBits::HISTORY,
            SkillType::Insight        => SkillBits::INSIGHT,
            SkillType::Intimidation   => SkillBits::INTIMIDATION,
            SkillType::Investigation  => SkillBits::INVESTIGATION,
            SkillType::Medicine       => SkillBits::MEDICINE,
            SkillType::Nature         => SkillBits::NATURE,
            SkillType::Perception     => SkillBits::PERCEPTION,
            SkillType::Performance    => SkillBits::PERFORMANCE,
            SkillType::Persuasion     => SkillBits::PERSUASION,
            SkillType::Religion       => SkillBits::RELIGION,
            SkillType::SleightOfHand  => SkillBits::SLEIGHT_OF_HAND,
            SkillType::Stealth        => SkillBits::STEALTH,
            SkillType::Survival       => SkillBits::SURVIVAL,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RaceType {
    Dwarf,
    Elf,
    Halfling,
    Human,
    Dragonborn,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
}

#[rustfmt::skip]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RaceBits: u16 {
        const DWARF      = 0b0000000000000001;
        const ELF        = 0b0000000000000010;
        const HALFLING   = 0b0000000000000100;
        const HUMAN      = 0b0000000000001000;
        const DRAGONBORN = 0b0000000000010000;
        const GNOME      = 0b0000000000100000;
        const HALF_ELF   = 0b0000000001000000;
        const HALF_ORC   = 0b0000000010000000;
        const TIEFLING   = 0b0000000100000000;
    }
}

#[rustfmt::skip]
impl From<RaceBits> for RaceType {
    fn from(value: RaceBits) -> Self {
        match value {
            RaceBits::DWARF      => RaceType::Dwarf,
            RaceBits::ELF        => RaceType::Elf,
            RaceBits::HALFLING   => RaceType::Halfling,
            RaceBits::HUMAN      => RaceType::Human,
            RaceBits::DRAGONBORN => RaceType::Dragonborn,
            RaceBits::GNOME      => RaceType::Gnome,
            RaceBits::HALF_ELF   => RaceType::HalfElf,
            RaceBits::HALF_ORC   => RaceType::HalfOrc,
            RaceBits::TIEFLING   => RaceType::Tiefling,
            _                    => unreachable!("Invalid race bits"),
        }
    }
}

#[rustfmt::skip]
impl From<RaceType> for RaceBits {
    fn from(value: RaceType) -> Self {
        match value {
            RaceType::Dwarf      => RaceBits::DWARF,
            RaceType::Elf        => RaceBits::ELF,
            RaceType::Halfling   => RaceBits::HALFLING,
            RaceType::Human      => RaceBits::HUMAN,
            RaceType::Dragonborn => RaceBits::DRAGONBORN,
            RaceType::Gnome      => RaceBits::GNOME,
            RaceType::HalfElf    => RaceBits::HALF_ELF,
            RaceType::HalfOrc    => RaceBits::HALF_ORC,
            RaceType::Tiefling   => RaceBits::TIEFLING,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
impl From<ClassBits> for ClassType {
    fn from(value: ClassBits) -> Self {
        match value {
            ClassBits::BARBARIAN => ClassType::Barbarian,
            ClassBits::BARD      => ClassType::Bard,
            ClassBits::CLERIC    => ClassType::Cleric,
            ClassBits::DRUID     => ClassType::Druid,
            ClassBits::FIGHTER   => ClassType::Fighter,
            ClassBits::MONK      => ClassType::Monk,
            ClassBits::PALADIN   => ClassType::Paladin,
            ClassBits::RANGER    => ClassType::Ranger,
            ClassBits::ROGUE     => ClassType::Rogue,
            ClassBits::SORCERER  => ClassType::Sorcerer,
            ClassBits::WARLOCK   => ClassType::Warlock,
            ClassBits::WIZARD    => ClassType::Wizard,
            _                    => unreachable!("Invalid class bits"),
        }
    }
}

#[rustfmt::skip]
impl From<ClassType> for ClassBits {
    fn from(value: ClassType) -> Self {
        match value {
            ClassType::Barbarian => ClassBits::BARBARIAN,
            ClassType::Bard      => ClassBits::BARD,
            ClassType::Cleric    => ClassBits::CLERIC,
            ClassType::Druid     => ClassBits::DRUID,
            ClassType::Fighter   => ClassBits::FIGHTER,
            ClassType::Monk      => ClassBits::MONK,
            ClassType::Paladin   => ClassBits::PALADIN,
            ClassType::Ranger    => ClassBits::RANGER,
            ClassType::Rogue     => ClassBits::ROGUE,
            ClassType::Sorcerer  => ClassBits::SORCERER,
            ClassType::Warlock   => ClassBits::WARLOCK,
            ClassType::Wizard    => ClassBits::WIZARD,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Proficiency {
    Armor(ArmorType),
    Weapon(WeaponType),
    Shield(ShieldType),
    SavingThrows(AbilityType),
    Skill(SkillType)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Proficiencies {
    armor: ArmorBits,
    weapon: WeaponBits,
    shield: ShieldBits,
    saving_throws: AbilityBits,
    skills: SkillBits,
}

impl Proficiencies {
    pub fn empty() -> Self {
        Proficiencies {
            armor: ArmorBits::empty(),
            weapon: WeaponBits::empty(),
            shield: ShieldBits::empty(),
            saving_throws: AbilityBits::empty(),
            skills: SkillBits::empty(),
        }
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
        self.armor.insert(ab);
    }

    #[inline]
    pub fn add_weapon(&mut self, weapon: WeaponType) {
        let wb = WeaponBits::from(weapon);
        self.weapon.insert(wb);
    }

    #[inline]
    pub fn add_shield(&mut self, shield: ShieldType) {
        let sb = ShieldBits::from(shield);
        self.shield.insert(sb);
    }

    #[inline]
    pub fn add_saving_throw(&mut self, ability: AbilityType) {
        let ab = AbilityBits::from(ability);
        self.saving_throws.insert(ab);
    }

    #[inline]
    pub fn add_skill(&mut self, skill: SkillType) {
        let sb = SkillBits::from(skill);
        self.skills.insert(sb);
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
    pub fn get_armor_list(&self) -> Vec<ArmorType> {
        return ArmorBits::FLAGS
            .iter()
            .filter(|&f| self.armor.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_weapon_list(&self) -> Vec<WeaponType> {
        return WeaponBits::FLAGS
            .iter()
            .filter(|&f| self.weapon.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_shield_list(&self) -> Vec<ShieldType> {
        return ShieldBits::FLAGS
            .iter()
            .filter(|&f| self.shield.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_saving_throw_list(&self) -> Vec<AbilityType> {
        return AbilityBits::FLAGS
            .iter()
            .filter(|&f| self.saving_throws.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_skill_list(&self) -> Vec<SkillType> {
        return SkillBits::FLAGS
            .iter()
            .filter(|&f| self.skills.contains(*(f.value())))
            .map(|f| (*f.value()).into())
            .collect::<Vec<_>>()
    }
}