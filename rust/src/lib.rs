#![feature(fn_traits)]
/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use log::Level;
use godot::prelude::*;
use crate::logger::{deinit_tracing, init_tracing, init_with_level};

mod hud;
mod main_scene;
mod mob;
mod player;
mod logger;
mod bullet;
pub mod pool;

struct DodgeTheCreeps;

#[gdextension]
unsafe impl ExtensionLibrary for DodgeTheCreeps {
    fn on_level_init(level: InitLevel) {
        tracing::info!("Init {:?}", level);
        match level {
            InitLevel::Core => {
                init_tracing();
                let _ = init_with_level(Level::Trace);
            }
            InitLevel::Servers => {}
            InitLevel::Scene => {}
            InitLevel::Editor => {}
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Core => deinit_tracing(),
            _ => tracing::info!("De-Init {:?}", level)
        }
    }
}
