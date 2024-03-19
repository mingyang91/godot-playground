use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct SightArea2D {
    base: Base<Area2D>,
}

#[godot_api]
impl SightArea2D {
    #[signal]
    fn character_in_sight(character: Gd<Area2D>);

    #[func]
    fn on_area_entered(&mut self, body: Gd<Area2D>) {
        self.base_mut().emit_signal("character_in_sight".into(), &[body.to_variant()]);
    }
}

#[godot_api]
impl IArea2D for SightArea2D {
    fn init(base: Base<Area2D>) -> Self {
        SightArea2D {
            base,
        }
    }

    fn ready(&mut self) {
        let listener = self.base().callable("on_area_entered");
        self.base_mut().connect("area_entered".into(), listener);
    }
}