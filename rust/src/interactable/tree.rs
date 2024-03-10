use godot::engine::Area2D;
use godot::prelude::*;
use crate::characters::goblin::Goblin;
use crate::dnd::enums::WeaponType;
use crate::interactable::{InteractWith};
use crate::tools::weapon::Weapon;

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
        let Ok(goblin) = body.try_cast::<Goblin>() else {
            return
        };

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


impl <W: Weapon> InteractWith<PineTree> for W {
    fn interact(&mut self, with: &mut PineTree) {
        let hp = match self.r#type() {
            WeaponType::SimpleMelee => 1,
            WeaponType::SimpleRanged => 0,
            WeaponType::MartialMelee => 2,
            WeaponType::MartialRanged => 0,
        };
        with.hp -= hp;
        if with.hp <= 0 {
            with.state = State::Stump;
        }
    }
}
