use bevy::app::{App, Plugin, Startup, Update};
use crate::debug::debug_components::{CursorDebugTextEvent, DrawGizmoEvent};
use crate::debug::debug_systems::*;

use self::debug_systems::move_debug_text_system;

mod debug_systems;
pub mod debug_components;

pub struct SmallDebugPlugin;

impl Plugin for SmallDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CursorDebugTextEvent>()
            .add_event::<DrawGizmoEvent>()
            .add_systems(Startup, debug_setup)
            // .add_systems(Update, experiiment)
            .add_systems(Update, move_debug_text_system)
            .add_systems(Update, change_debug_text_system)
            .add_systems(Update, cursor_position_debug_system)
            .add_systems(Update, draw_belt_center)
            .add_systems(Update, draw_gizmo_event_system)

        // .add_systems(Update, debug_draw_conveyors)

            // .add_systems(Update, draw_belt_forward)
            ;
    }
}
