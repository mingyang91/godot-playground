use godot::engine::{AnimationTree, IAnimationTree};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=AnimationTree)]
struct GoblinAnimationTree {
    base: Base<AnimationTree>,
}


impl GoblinAnimationTree {

}

#[godot_api]
impl IAnimationTree for GoblinAnimationTree {
    fn init(base: Base<AnimationTree>) -> Self {
        GoblinAnimationTree {
            base,
        }
    }

    fn ready(&mut self) {
        let mut anim_player = self.base_mut()
            .set("parameters/conditions/is_idle".into(), true.to_variant());
    }
}