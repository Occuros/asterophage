use std::cmp::max;
use bevy::math::{Quat, Vec2};
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use crate::debug::debug_components::CursorDebugTextEvent;
use std::f32::consts::TAU;
use crate::world_grid::components::{Cell, GridPosition, GroundLayer, SurfaceLayer, WorldGrid};
use bevy_vector_shapes::prelude::*;
use crate::player::player_components::{GameCursor, Player};


pub fn draw_grid(
    _commands: Commands,
    mut painter: ShapePainter,
    world_grid: Res<WorldGrid>,
    game_cursor: Res<GameCursor>,
    player_q: Query<&Transform, With<Player>>,
    _asset_server: Res<AssetServer>,
    mut debut_event: EventWriter<CursorDebugTextEvent>,
) {
    if player_q.get_single().is_err() { return; }
    let rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.set_rotation(rotation);
    painter.thickness = 0.01;
    let cursor_grid_position = world_grid
        .get_grid_position_from_world_position(game_cursor.world_position.unwrap_or_default());

    let player_transform = player_q.single();
    let center = world_grid.get_grid_position_from_world_position(player_transform.translation);
    let draw_distance = 20;

    for x in center.x - draw_distance..=center.x + draw_distance {
        for y in center.y - draw_distance..=center.y + draw_distance {
            let grid_position = GridPosition { x, y };
            let cell_selected = cursor_grid_position == grid_position;
            let mut position = world_grid.grid_to_world(&grid_position);
            let max_distance = (draw_distance - 5) as f32 * world_grid.grid_size;
            painter.hollow = !cell_selected;
            painter.color = if cell_selected {
                let distance_x = 1.0 - (position.x - player_transform.translation.x).abs() / max_distance;
                let distance_y = 1.0 - (position.z - player_transform.translation.z).abs() / max_distance;
                let distance = distance_x.min(distance_y);
                debut_event.send(CursorDebugTextEvent { text: format!("distance: {}", distance) });
                Color::BLACK
            } else {
                let distance_x = 1.0 - (position.x - player_transform.translation.x).abs() / max_distance;
                let distance_y = 1.0 - (position.z - player_transform.translation.z).abs() / max_distance;
                let distance = distance_x.min(distance_y);
                let grey_value = 0.8;
                Color::rgba(grey_value, grey_value, grey_value, distance)
            };

            position.y += 0.001;
            painter.transform.translation = position;
            // painter.circle(world_grid.grid_size * 0.5);
            painter.rect(Vec2::splat(world_grid.grid_size));
        }
    }
}

pub fn discover_world_system(
    commands: Commands,
    mut painter: ShapePainter,
    mut world_grid: ResMut<WorldGrid>,
    game_cursor: Res<GameCursor>,
    player_q: Query<&Transform, With<Player>>,
    _asset_server: Res<AssetServer>,
) {
    if player_q.get_single().is_err() { return; }
    let rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.set_rotation(rotation);
    painter.thickness = 0.01;
    let cursor_grid_position = world_grid
        .get_grid_position_from_world_position(game_cursor.world_position.unwrap_or_default());

    let player_transform = player_q.single();
    let center = world_grid.get_grid_position_from_world_position(player_transform.translation);
    let draw_distance = 20;

    for x in center.x - draw_distance..=center.x + draw_distance {
        for y in center.y - draw_distance..=center.y + draw_distance {
            let grid_position = GridPosition { x, y };

            // let mut position = world_grid.grid_to_world(&grid_position);
            // let max_distance = (draw_distance - 5) as f32 * world_grid.grid_size;
            match  world_grid.cells.get(&grid_position) {
                None => {
                    info!("cell discoverd at {:?}", grid_position);
                    world_grid.cells.insert(grid_position, Cell {
                        ground_layer: GroundLayer::Empty,
                        surface_layer: SurfaceLayer::Empty,
                    });
                }
                _ => {}
            }
        }
    }
}
