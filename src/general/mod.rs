use bevy::prelude::*;
use bevy_editor_pls::editor::Editor;
use crate::general::general_systems::*;
use crate::{MainCamera, setup};
use crate::general::general_components::{Gizmodius, SpatiallyTracked};
use crate::player::player_components::GameCursor;

mod general_systems;
pub mod general_components;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SpatiallyTracked>()
            .insert_resource(GameCursor::default())
            .init_gizmo_group::<Gizmodius>()

            .add_systems(PostUpdate, update_cursor_system)
            .add_systems(Update, button_highlight_system)
            .add_systems(Update, building_ui_selection_system)
            .add_systems(Update, remove_preview_building_system)
            .add_systems(Update, rotate_preview_item_system)
            .add_systems(PostUpdate , move_building_preview_with_cursor_system)
            .add_systems(Startup, setup_menu.after(setup))
        ;
    }
}

fn editor_not_active(editor: Res<Editor>) ->bool {
    !editor.active()
}

pub trait Pastel {
    fn pastel(&self) -> Color;
}

impl Pastel for Color {
    fn pastel(&self) -> Color {
        (*self + Color::WHITE * 0.25).with_a(1.0)
    }
}