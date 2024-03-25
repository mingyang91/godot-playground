use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use godot::engine::NavigationAgent2D;
use godot::prelude::*;

#[derive(Debug)]
pub struct Navigator {
    navigation_agent: Gd<NavigationAgent2D>,
    target: Option<Gd<Node2D>>
}

impl Navigator {
    pub fn new(navigation_agent: Gd<NavigationAgent2D>) -> Self {
        Navigator {
            navigation_agent,
            target: None,
        }
    }

    #[inline]
    pub fn navigate_to(&mut self, target: Vector2) {
        self.navigation_agent.borrow_mut().set_target_position(target);
    }

    pub fn follow(&mut self, target: Gd<Node2D>) {
        self.target = Some(target);
    }

    pub fn get_target(&self) -> Option<Gd<Node2D>> {
        self.target.clone()
    }

    pub fn stop_following(&mut self) {
        self.target = None;
    }

    #[inline]
    pub fn update(&mut self) {
        if let Some(target) = &self.target {
            self.navigate_to(target.get_position());
        }
    }

    pub fn is_following(&self) -> bool {
        self.target.is_some()
    }

    #[inline]
    pub fn get_next_position(&mut self) -> Vector2 {
        self.update();
        self.navigation_agent.get_next_path_position()
    }
}

impl Deref for Navigator {
    type Target = NavigationAgent2D;

    fn deref(&self) -> &Self::Target {
        &self.navigation_agent
    }
}

impl DerefMut for Navigator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.navigation_agent
    }
}