use crate::building::building_components::*;
use crate::player::player_components::GameCursor;
use crate::world_grid::world_gird_components::{SurfaceLayer, WorldGrid};
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;



pub fn place_building_system(
    mut commands: Commands,
    mut shapes: ShapeCommands,
    mut asset_server: ResMut<AssetServer>,
    input: Res<Input<MouseButton>>,
    game_cursor: ResMut<GameCursor>,
    mut world_grid: ResMut<WorldGrid>,
    mut building_q: Query<(Entity, &Transform, &Building)>,
    requires_ground_q: Query<&RequiresGround>,
) {
    if game_cursor.world_position.is_none() {
        return;
    };
    if game_cursor.preview_entity.is_none() {
        return;
    };
    if !input.just_pressed(MouseButton::Left) {
        return;
    }
    let grid_size = world_grid.grid_size;
    let position = game_cursor.world_position.unwrap();

    let grid_position = world_grid.get_grid_position_from_world_position(position);
    let building_position = world_grid.grid_to_world(&grid_position);

    if let Some(cell) = world_grid.cells.get_mut(&grid_position) {
        if cell.surface_layer != SurfaceLayer::Empty {
            return;
        };

        let preview_entity = game_cursor.preview_entity.unwrap();

        if building_q.get(preview_entity).is_err() {
            return;
        };
        let (entity, transform, building) = building_q.get_mut(preview_entity).unwrap();
        if let Ok(requires_ground) = requires_ground_q.get(entity) {
            if !requires_ground.allowed_ground.contains(&cell.ground_layer) {
                return;
            }
        }

        let placed_building = Building::spawn(building.building_type, building_position, transform.rotation, grid_size, &mut commands, &mut asset_server, &mut shapes);

        cell.surface_layer = SurfaceLayer::Building {
            entity: placed_building.unwrap(),
        };
   
    
    }
}


