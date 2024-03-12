use godot::obj::NewAlloc;
use godot::prelude::{Base, Gd, godot_api, GodotClass, IObject, Object};

#[derive(GodotClass, Debug)]
#[class(base=Object)]
pub struct Effects {
    pub effects: Vec<Effect>,
    base: Base<Object>,
}

#[derive(Clone, Debug)]
pub enum Effect {
    Damage(Damage),
    Heal(Heal),
    Buff(Buff),
    DeBuff(DeBuff),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Damage {
    pub amount: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Heal {
    pub amount: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Buff {
    pub duration: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeBuff {
    pub duration: i32,
}

#[godot_api]
impl Effects {
    pub fn new(effects: Vec<Effect>) -> Gd<Self> {
        let mut this = Self::new_alloc();
        this.bind_mut().effects = effects;
        this
    }
}

#[godot_api]
impl IObject for Effects {
    fn init(base: Base<Object>) -> Self {
        Effects {
            effects: Vec::new(),
            base,
        }
    }
}