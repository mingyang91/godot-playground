use crate::dnd::enums::WeaponType;

pub trait Weapon {
    fn r#type(&self) -> WeaponType;
    fn damage(&self) -> u32;
    fn range(&self) -> u32;
}

pub trait SimpleMeleeWeapon: Weapon {
    fn r#type(&self) -> WeaponType {
        WeaponType::SimpleMelee
    }
}

pub trait SimpleRangedWeapon: Weapon {
    fn r#type() -> WeaponType {
        WeaponType::SimpleRanged
    }
    fn ammunition(&self) -> u32;
}

pub trait MartialMeleeWeapon: Weapon {
    fn r#type() -> WeaponType {
        WeaponType::MartialMelee
    }
}

pub trait MartialRangedWeapon: Weapon {
    fn r#type() -> WeaponType {
        WeaponType::MartialRanged
    }
    fn ammunition(&self) -> u32;
}