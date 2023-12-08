use crate::building::building_components::*;
use crate::player::player_components::GameCursor;
use crate::world_grid::world_gird_components::{SurfaceLayer, WorldGrid};
use bevy::prelude::*;

pub fn place_building_system(
    input: Res<Input<MouseButton>>,
    mut game_cursor: ResMut<GameCursor>,
    mut world_grid: ResMut<WorldGrid>,
    mut building_q: Query<(&mut Transform, &RequiresGround), With<Building>>,
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
        let (mut transform, requires_ground) = building_q.get_mut(preview_entity).unwrap();
        
        if !requires_ground.allowed_ground.contains(&cell.ground_layer) {
            return;
        }

        cell.surface_layer = SurfaceLayer::Building {
            entity: game_cursor.preview_entity.unwrap(),
        };
        transform.translation = building_position;
        game_cursor.preview_entity = None;
    }
}

