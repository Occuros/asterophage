use bevy::app::{App, Plugin, Startup, Update};
use crate::debug::debug_components::CursorDebugTextEvent;
use crate::debug::debug_systems::{change_debug_text_system, debug_setup};
use crate::player::PlayerPlugin;

use self::debug_systems::move_debug_text_system;

mod debug_systems;
pub mod debug_components;

pub struct SmallDebugPlugin;

impl Plugin for SmallDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CursorDebugTextEvent>()
            .add_systems(Startup, debug_setup)
            .add_systems(Update, move_debug_text_system)
            .add_systems(Update, change_debug_text_system)
            ;
    }
}
