use bevy::prelude::*;
use crate::world_grid::components::{Cell, GridPosition, WorldGrid};
use crate::world_grid::systems::*;

pub mod components;
mod systems;

pub struct WorldGridPlugin;

impl Plugin for WorldGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GridPosition>()
            .register_type::<Cell>()
            .register_type::<WorldGrid>()
            .insert_resource(WorldGrid::new(20, 10, 1.0))
            .add_systems(Startup, debug_world_system)
            .add_systems(Startup, debug_spawn_grid_positions)
            // .add_systems(Startup, gird_test_system.before(debug_spawn_grid_positions))
            .add_systems(Update, update_grid_positions)
            .add_systems(Update, draw_grid)
        ;
    }
}