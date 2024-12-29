use crate::general::general_systems::*;
use crate::player::player_components::GameCursor;
use crate::setup;
use bevy::prelude::*;

mod general_components;
mod general_systems;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameCursor::default())
            .add_systems(PostUpdate, update_cursor_system)
            .add_systems(Update, button_highlight_system)
            .add_systems(Update, building_ui_selection_system)
            .add_systems(Update, remove_preview_building_system)
            .add_systems(Update, rotate_preview_item_system)
            .add_systems(PostUpdate, move_building_preview_with_cursor_system)
            .add_systems(Startup, setup_menu.after(setup));
    }
}
