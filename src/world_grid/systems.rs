use bevy::math::{Quat, Vec2};
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use std::f32::consts::TAU;
use crate::world_grid::components::{Cell, GridPosition, WorldGrid};
use bevy_vector_shapes::prelude::*;
use crate::player::player_components::GameCursor;

pub fn debug_world_system(world_grid: Res<WorldGrid>, _painter: ShapePainter) {
    println!(
        "world gird size: w:{} h:{}",
        world_grid.width, world_grid.height
    );
    // for cell in &world_grid {
    //     println!("cells: x:{}, y:{}", cell.position.x, cell.position.y);
    // }
}

pub fn debug_spawn_grid_positions(
    mut commands: Commands,
    world_grid: Res<WorldGrid>,
    asset_server: Res<AssetServer>,
) {
    for (grid_position, cell) in world_grid.into_iter() {
        let mut position = world_grid.grid_to_world(&grid_position);
        position.y += 0.1;
        // let rotation = Quat::from_rotation_x(TAU * 0.3);
        commands.spawn((
            BillboardTextBundle {
                transform: Transform::from_translation(position)
                    // .with_rotation(rotation)
                    .with_scale(Vec3::splat(0.01)),
                text: Text::from_sections([TextSection {
                    value: format!("{:?}", cell),
                    style: TextStyle {
                        font_size: 30.0,
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        color: Color::WHITE,
                    },
                }])
                    .with_alignment(TextAlignment::Center),
                ..default()
            },
            GridPosition {
                x: grid_position.x,
                y: grid_position.y,
            },
            Cell::default(),
        ));
    }
}

pub fn update_grid_positions(
    mut commands: Commands,
    world_grid: Res<WorldGrid>,
    mut grid_query: Query<(Entity, &mut Text, &Cell, &GridPosition)>,
) {
    for (entity, mut text, cell, grid_position) in &mut grid_query {
        let updated_cell = &world_grid.cells[grid_position];
        if cell != updated_cell {
            commands.entity(entity).insert(updated_cell.clone());
            text.sections[0].value = format!("{:?}", updated_cell);
        }
    }
}

pub fn draw_grid(
    _commands: Commands,
    mut painter: ShapePainter,
    world_grid: Res<WorldGrid>,
    game_cursor: Res<GameCursor>,
    _asset_server: Res<AssetServer>,
) {
    let rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.set_rotation(rotation);
    painter.thickness = 0.01;
    let cursor_grid_position = world_grid
        .get_grid_position_from_world_position(game_cursor.world_position.unwrap_or_default());
    for (grid_position, _cell) in &world_grid {
        let cell_selected = cursor_grid_position == grid_position;
        let mut position = world_grid.grid_to_world(&grid_position);

        painter.hollow = !cell_selected;
        painter.color = if cell_selected {
            Color::GRAY
        } else {
            Color::WHITE
        };

        position.y += 0.001;
        painter.transform.translation = position;
        // painter.circle(world_grid.grid_size * 0.5);
        painter.rect(Vec2::splat(world_grid.grid_size));
    }
}

#[allow(dead_code)]
pub fn gird_test_system(mut word_grid: ResMut<WorldGrid>) {
    word_grid.set_cell(Cell::IntCell {
        number: 5,
    }, GridPosition { x: 1, y: 1 })
}
