use crate::building::conveyor_belt::ConveyorBelt;
use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy::ui::Val::Px;
use bevy_vector_shapes::painter::ShapePainter;
use bevy_vector_shapes::prelude::*;
use crate::building::building_components::*;
use crate::debug::debug_components::*;
use crate::general::Pastel;
use crate::player::player_components::GameCursor;
use crate::world_grid::world_gird_components::*;

pub fn debug_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 10.0,
                ..default()
            },
        ).with_text_justify(JustifyText::Left)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            }),
        DebugText,
    ));
}

pub fn cursor_position_debug_system(
    game_cursor: Res<GameCursor>,
    mut debug_text_event: EventWriter<CursorDebugTextEvent>,
    world_grid: Res<WorldGrid>,
) {
    let position = world_grid.get_grid_position_from_world_position(game_cursor.world_position.unwrap_or_default());
    debug_text_event.send(CursorDebugTextEvent{
        text: format!("x:{} y:{} => {:.2?}", position.x, position.y, game_cursor.world_position.unwrap_or_default()),
    });
}

pub fn move_debug_text_system
(
    window_query: Query<&Window>,
    mut debug_text_q: Query<&mut Style, With<DebugText>>,
) {
    let window = window_query.get_single().unwrap();
    if let Some(cursor_position) = window.cursor_position() {
        let mut style = debug_text_q.single_mut();
        style.left = Px(cursor_position.x + 20.0);
        style.top = Px(cursor_position.y + 20.0);
    }
}

pub fn change_debug_text_system(
    mut debug_text_event: EventReader<CursorDebugTextEvent>,
    mut debut_text_q: Query<&mut Text, With<DebugText>>,
) {
    if debut_text_q.get_single().is_err() {return;}
    let mut text: Mut<'_, Text> = debut_text_q.single_mut();

    for event in debug_text_event.read() {
        text.sections[0].value = event.text.to_owned();
    }
}
pub fn debug_draw_conveyors(
    mut shapes: ShapePainter,
    conveyor_q: Query<&ConveyorBelt>,
    world_grid: Res<WorldGrid>,
) {
    for conveyor in conveyor_q.iter() {
        for belt in conveyor.belt_pieces.iter() {
            shapes.transform = Transform::from_translation(
                world_grid.grid_to_world(&belt.grid_position) + Vec3::Y * 0.15,
            )
                .with_rotation(Quat::from_rotation_x(TAU * 0.25));
            shapes.thickness = 0.01;
            shapes.hollow = true;
            // shapes.color = Color::PURPLE.pastel();

            if belt.grid_position == conveyor.start_position() {
                shapes.color = Color::BLACK;
                shapes.circle(0.1);
            }
            if belt.grid_position == conveyor.end_position() {
                shapes.color = Color::RED;
                shapes.circle(0.15);
            }
            // shapes.rect(Vec2::splat(0.9 * world_grid.grid_size));

            for segment in &conveyor.segments {
                info_once!("Drawing segment {:?}", segment);
                // shapes.transform = Transform::from_translation(Vec3::Y * 0.15);
                let offset = Vec3::Y * 0.15;
                shapes.color = Color::PINK;
                shapes.thickness = 0.1;
                shapes.hollow = false;
                shapes.transform = Transform::from_translation( segment.start_position + offset).with_rotation(Quat::from_rotation_x(TAU * 0.25));
                shapes.circle(0.1);

                shapes.color = Color::PURPLE;

                shapes.transform = Transform::from_translation( segment.end_position + offset).with_rotation(Quat::from_rotation_x(TAU * 0.25));;

                shapes.circle(0.05);
                shapes.transform = Transform::from_translation( Vec3::Y * 0.12);;
                shapes.alignment = Alignment::Billboard;
                shapes.thickness = 0.02;
                shapes.line(segment.start_position, segment.end_position);
            }

        }
    }
}
