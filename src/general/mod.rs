use bevy::prelude::*;
use crate::general::general_systems::*;
use crate::{MainCamera, setup};
use crate::player::player_components::GameCursor;

mod general_systems;
mod general_components;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameCursor::default())
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


pub trait Pastel {
    fn pastel(&self) -> Color;
}

impl Pastel for Color {
    fn pastel(&self) -> Color {
        (*self + Color::WHITE * 0.25).with_a(1.0)
    }
}