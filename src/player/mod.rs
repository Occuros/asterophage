use bevy::prelude::*;
use crate::AppState;
use crate::player::player_systems::*;

pub mod player_components;
mod player_systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game),spawn_player)
            .add_systems(
                Update,(
                    move_player,
                    move_camera_system.after(move_player),
                    move_light_system.after(move_player),
                    // shoot,
                    life_time_system,
                    bullet_collisions_system,
                ).run_if(in_state(AppState::Game))
            )
            // .add_systems(PostUpdate, paint_target)
        ;
    }
}