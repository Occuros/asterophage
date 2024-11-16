use crate::player::player_components::{GameCursor, Player};
use crate::world_grid::components::yellow_bile::YellowBileResource;
use crate::world_grid::world_gird_components::*;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;
use std::f32::consts::TAU;

pub fn draw_grid(
    _commands: Commands,
    mut painter: ShapePainter,
    world_grid: Res<WorldGrid>,
    game_cursor: Res<GameCursor>,
    player_q: Query<&Transform, With<Player>>,
) {
    if player_q.get_single().is_err() {
        return;
    }
    let rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.set_rotation(rotation);
    painter.thickness = 0.01;
    let cursor_grid_position = world_grid
        .get_grid_position_from_world_position(game_cursor.world_position.unwrap_or_default());

    let player_transform = player_q.single();
    let center = world_grid.get_grid_position_from_world_position(player_transform.translation);
    let draw_distance = 50;

    for x in center.x - draw_distance..=center.x + draw_distance {
        for y in center.y - draw_distance..=center.y + draw_distance {
            let grid_position = GridPosition { x, y };
            let cell_selected = cursor_grid_position == grid_position;
            let mut position = world_grid.grid_to_world(&grid_position);
            let max_distance = (draw_distance - 5) as f32 * world_grid.grid_size;
            painter.hollow = !cell_selected;
            painter.color = if cell_selected {
                position.y += 0.1;
                let grey_value = 0.6;
                Color::srgba(grey_value, grey_value, grey_value, 0.3)
            } else {
                let distance_x =
                    1.0 - (position.x - player_transform.translation.x).abs() / max_distance;
                let distance_y =
                    1.0 - (position.z - player_transform.translation.z).abs() / max_distance;
                let distance = distance_x.min(distance_y) * 0.1;
                let grey_value = 0.5;
                position.y += 0.001;
                Color::rgba(grey_value, grey_value, grey_value, distance)
            };

            painter.transform.translation = position;
            // painter.circle(world_grid.grid_size * 0.5);
            painter.rect(Vec2::splat(world_grid.grid_size));
            // let zoom_level = resource_settings.zoom_level;
            // let noise_value = get_noise_value(grid_position, zoom_level);
            // painter.hollow = false;
            // painter.color = Color::rgb(noise_value, noise_value, noise_value);
            // painter.rect(Vec2::splat(world_grid.grid_size));
        }
    }
}

fn get_noise_value(grid_position: GridPosition, zoom_level: f32) -> f32 {
    let x = grid_position.x as f32;
    let y = grid_position.y as f32;
    let frequencies = vec![1.0, 0.5, 0.25];
    let mut combined_noise = 0.0;
    for f in &frequencies {
        combined_noise += f
            * ((simplex_noise_2d_seeded(Vec2::new(f * x * zoom_level, f * y * zoom_level), *f)
                + 1.0)
                * 0.5)
    }
    combined_noise / frequencies.iter().sum::<f32>()
}

pub fn discover_world_system(
    mut commands: Commands,
    mut painter: ShapePainter,
    mut world_grid: ResMut<WorldGrid>,
    player_q: Query<&Transform, With<Player>>,
    mut asset_server: ResMut<AssetServer>,
    resource_settings: Res<ResourceNoiseSettings>,
) {
    if player_q.get_single().is_err() {
        return;
    }
    let rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.set_rotation(rotation);
    painter.thickness = 0.01;

    let player_transform = player_q.single();
    let center = world_grid.get_grid_position_from_world_position(player_transform.translation);
    let draw_distance = 20;

    for x in center.x - draw_distance..=center.x + draw_distance {
        for y in center.y - draw_distance..=center.y + draw_distance {
            let grid_position = GridPosition { x, y };

            if world_grid.cells.get(&grid_position).is_none() {
                if get_noise_value(grid_position, resource_settings.zoom_level)
                    > resource_settings.bile_level
                {
                    let position = world_grid.grid_to_world(&grid_position);
                    YellowBileResource::spawn(
                        position,
                        Quat::default(),
                        world_grid.grid_size,
                        100,
                        &mut commands,
                        &mut asset_server,
                    );
                    world_grid.cells.insert(
                        grid_position,
                        Cell {
                            ground_layer: GroundLayerType::YellowBileResource,
                            surface_layer: SurfaceLayer::Empty,
                            item_layer: ItemLayer::Empty,
                        },
                    );
                } else {
                    world_grid.cells.insert(
                        grid_position,
                        Cell {
                            ground_layer: GroundLayerType::Empty,
                            surface_layer: SurfaceLayer::Empty,
                            item_layer: ItemLayer::Empty,
                        },
                    );
                }
            }
        }
    }
}
