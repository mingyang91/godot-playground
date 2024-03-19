use godot::engine::{AnimatedSprite2D};
use godot::prelude::*;
use crate::interactable::effect::{Effect, Effects};
use crate::interactable::hurt_box::HurtBox;

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

    sprite: Option<Gd<AnimatedSprite2D>>,
    hurt_box: Option<Gd<HurtBox>>,
    base: Base<Node2D>
}

#[godot_api]
impl PineTree {
    #[func]
    fn hurt(&mut self, effects: Gd<Effects>) {
        tracing::debug!("hurt: {:?}", effects);

        let Some(_has_damage) = effects.bind().effects.iter().find(|&eff| {
            match eff {
                Effect::Damage(_) => true,
                _ => false
            }
        }) else {
            return
        };
        self.hp -= 1;

        if self.hp <= 0 {
            self.state = State::Stump;
            self.sprite.as_mut().expect("must have").play_ex().name("stump".into()).done();
        } else {
            self.sprite.as_mut().expect("must have").play_ex().name("chopping".into()).done();
        }
    }
}

#[godot_api]
impl INode2D for PineTree {
    fn init(base: Base<Node2D>) -> Self {
        PineTree {
            hp: 3,
            state: State::Idle,
            sprite: None,
            hurt_box: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.state = State::Idle;

        let base = self.base_mut();
        let hurt_box = base.get_node_as::<HurtBox>("HurtBox");
        let mut sprite = base.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite.play();

        drop(base);

        self.hurt_box = Some(hurt_box.clone());
        self.sprite = Some(sprite.clone());
    }
}

