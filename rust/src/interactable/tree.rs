use godot::engine::Area2D;
use godot::prelude::*;

enum State {
    Idle,
    Stump,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PineTree {
    #[var]
    hp: i32,
    state: State,

    base: Base<Node2D>
}

#[godot_api]
impl PineTree {
    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        tracing::info!("body entered: {:?}", body);
        self.hp -= 1;
        if self.hp <= 0 {
            self.state = State::Stump;
        }
    }
}

#[godot_api]
impl INode2D for PineTree {
    fn init(base: Base<Node2D>) -> Self {
        PineTree {
            hp: 3,
            state: State::Idle,
            base,
        }
    }

    fn ready(&mut self) {
        let hp = self.hp;
        self.state = State::Idle;
    }
}
