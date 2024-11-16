use self::debug_systems::move_debug_text_system;
use crate::debug::debug_components::CursorDebugTextEvent;
use crate::debug::debug_systems::*;
use crate::setup;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::IntoSystemConfigs;

pub mod debug_components;
mod debug_systems;

pub struct SmallDebugPlugin;

impl Plugin for SmallDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CursorDebugTextEvent>()
            .add_systems(Startup, debug_setup.after(setup))
            .add_systems(Update, move_debug_text_system)
            .add_systems(Update, change_debug_text_system)
            .add_systems(Update, cursor_position_debug_system)
            .add_systems(Update, debug_draw_conveyors)

            // .add_systems(Update, draw_belt_forward)
            ;
    }
}
