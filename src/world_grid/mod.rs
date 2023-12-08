use bevy::prelude::*;
use crate::world_grid::world_gird_components::{Cell, GridPosition, ResourceNoiseSettings, WorldGrid};
use crate::world_grid::world_grid_systems::*;

pub mod world_gird_components;
mod world_grid_systems;

pub struct WorldGridPlugin;

impl Plugin for WorldGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GridPosition>()
            .register_type::<Cell>()
            .register_type::<WorldGrid>()
            .insert_resource(WorldGrid::new( 0.5))
            .insert_resource(ResourceNoiseSettings {
                zoom_level: 0.02,
                bile_level: 0.83,
            })
            // .add_systems(Startup, debug_world_system)
            // .add_systems(Startup, debug_spawn_grid_positions)
            // .add_systems(Startup, gird_test_system.before(debug_spawn_grid_positions))
            .add_systems(Update, draw_grid)
            .add_systems(Update, discover_world_system)
        ;
    }
}