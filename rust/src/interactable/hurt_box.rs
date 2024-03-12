use godot::engine::{Area2D, CollisionShape2D, IArea2D};
use godot::prelude::*;
use crate::interactable::effect::Effects;
use crate::interactable::hit_box::HitBox;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct HurtBox {
    shape: Option<Gd<CollisionShape2D>>,
    base: Base<Area2D>
}

#[godot_api]
impl HurtBox {
    #[signal]
    fn hurt(effects: Gd<Effects>);

    #[func]
    fn on_area_entered(&mut self, body: Gd<HitBox>) {
        tracing::debug!("hit box entered: {:?}", body);
        let effects = body.bind().get_effects();
        self.base_mut().emit_signal("hurt".into(), &[effects.to_variant()]);
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        let Some(shape) = self.shape.as_mut() else {
            return
        };
        shape.set_disabled(disabled);
    }
}

#[godot_api]
impl IArea2D for HurtBox {
    fn init(base: Base<Area2D>) -> Self {
        HurtBox {
            shape: None,
            base,
        }
    }

    fn ready(&mut self) {
        let listener = self.base().callable("on_area_entered");
        self.base_mut().connect("area_entered".into(), listener);
    }

    fn get_configuration_warnings(&self) -> PackedStringArray {
        let mut warning = PackedStringArray::new();
        if self.shape.is_none() {
            let msg = "HurtBox has no shape, consider adding a CollisionShape2D";
            warning.push(msg.into())
        }
        warning
    }
}
