use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;
use crate::interactable::effect::Effects;
use crate::interactable::hurt_box::HurtBox;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct HitBox {
    effects: Gd<Effects>,

    base: Base<Area2D>
}

#[godot_api]
impl HitBox {
    pub fn get_effects(&self) -> Gd<Effects> {
        self.effects.clone()
    }

    pub fn set_effects(&mut self, effects: Gd<Effects>) {
        self.effects = effects;
    }

    #[func]
    fn on_area_entered(&mut self, body: Gd<HurtBox>) {
        tracing::debug!("hurt box entered: {:?}", body);
    }
}

#[godot_api]
impl IArea2D for HitBox {
    fn init(base: Base<Area2D>) -> Self {
        HitBox {
            effects: Effects::new_alloc(),
            base,
        }
    }

    fn ready(&mut self) {
        let listener = self.base().callable("on_area_entered");
        self.base_mut().connect("area_entered".into(), listener);
    }
}