use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy::ui::Val::Px;
use bevy_vector_shapes::painter::ShapePainter;
use bevy_vector_shapes::prelude::*;
use log::Level::Debug;
use crate::building::building_components::{BeltElement, ConveyorBelt, Preview};
use crate::debug::debug_components::*;
use crate::general::general_components::Gizmodius;
use crate::general::Pastel;
use crate::player::player_components::GameCursor;
use crate::world_grid::world_gird_components::{GridPiece, WorldGrid};

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
    debug_text_event.send(CursorDebugTextEvent {
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
    if debut_text_q.get_single().is_err() { return; }
    let mut text: Mut<'_, Text> = debut_text_q.single_mut();

    for event in debug_text_event.read() {
        text.sections[0].value = event.text.to_owned();
    }
}

pub fn draw_belt_forward(
    belt_q: Query<&Transform, With<BeltElement>>,
    mut painter: ShapePainter,
    world_grid: Res<WorldGrid>,
) {
    // painter.thickness = 0.5;
    // painter.color = Color::MIDNIGHT_BLUE;
    // painter.cap = Cap::None;
    // painter.line(Vec3::new(-1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0));

    painter.line(Vec3::new(-1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    for t in belt_q.iter() {
        painter.reset();
        painter.color = Color::WHITE;
        painter.thickness = 0.1;
        painter.cap = Cap::Round;
        painter.transform.translation = t.translation + Vec3::Y * 0.01 + *t.local_z() * 0.1;
        painter.transform.rotation = t.rotation;
        let start = Vec3::ZERO + Vec3::Y * 0.01;
        let end = start + Vec3::Z * 1.0;
        painter.transform.rotation *= Quat::from_rotation_x(TAU * 0.25);

        painter.rect(Vec2::new(0.1, 0.4));

        // painter.transform.translation = start;
        // painter.circle(0.1);
        // painter.color = Color::MIDNIGHT_BLUE;
        // painter.transform.translation = end;
        // painter.circle(0.15);
        // let grid = world_grid.get_grid_position_from_world_position(t.translation);
        // let grid_forward = grid.get_relative_forward(t.grid_rotation());
        //
        // let start = world_grid.grid_to_world(&grid) + Vec3::Y * 0.01;
        // let end = world_grid.grid_to_world(&grid_forward) + Vec3::Y * 0.01;
        //
        // painter.line(start, end);

        // painter.transform.translation = start;
        // painter.circle(0.1);
        // painter.color = Color::MIDNIGHT_BLUE;
        // painter.transform.translation = end;
        // painter.circle(0.15);
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
                world_grid.grid_to_world(&belt.grid_position) + Vec3::Y * 0.05,
            )
                .with_rotation(Quat::from_rotation_x(TAU * 0.25));
            shapes.thickness = 0.02;
            shapes.hollow = true;

            if belt.grid_position == conveyor.start_position() {
                shapes.color = Color::BLACK;
                shapes.circle(0.1);
            }
            if belt.grid_position == conveyor.end_position() {
                shapes.color = Color::RED;
                shapes.circle(0.15);
            }
            shapes.color = Color::PURPLE.pastel();
            shapes.rect(Vec2::splat(0.9 * world_grid.grid_size));
        }
    }
}

pub fn draw_belt_center(
    belt_q: Query<(&BeltElement, &Transform, Option<&Preview>)>,
    mut gizmos: Gizmos<Gizmodius>,
) {
    for (belt, trans, preview) in belt_q.iter() {
        if belt.is_corner {
            gizmos.sphere(belt.center, Quat::IDENTITY, 0.1, Color::BLACK);
        }
        // if preview.is_some() {
        //     let right_position = trans.translation + trans.right() * 0.5 - Vec3::Y * 0.1;
        //     gizmos.sphere(right_position, Quat::IDENTITY, 0.1, Color::BLACK);
        // }
    }
}

pub fn draw_gizmo_event_system(
    mut gizmo_events: EventReader<DrawGizmoEvent>,
    mut gizmos: Gizmos<Gizmodius>,
    mut drawings: Local<Vec<DrawGizmoEvent>>,
    time: Res<Time>,
) {
    for gizmo_event in gizmo_events.read() {
        draw_gizmo(&mut gizmos, gizmo_event);
        drawings.push(gizmo_event.clone());
    }

    for d in drawings.iter_mut() {
        draw_gizmo(&mut gizmos, d);
        match d {
            DrawGizmoEvent::Sphere { timer, .. } => {
                timer.tick(time.delta());
            }
            DrawGizmoEvent::Arrow { timer,.. } => {
                timer.tick(time.delta());
            }
        };
    };

    drawings.retain(|d| {
        match d {
            DrawGizmoEvent::Sphere { timer, .. } => {
                !timer.finished()
            }
            DrawGizmoEvent::Arrow { timer,..
            } => {
                !timer.finished()
            }
        }
    });

    fn draw_gizmo(gizmos: &mut Gizmos<Gizmodius>, gizmo_event: &DrawGizmoEvent) {
        match *gizmo_event {
            DrawGizmoEvent::Sphere {
                radius,
                color,
                position,
                ..
            } => {
                gizmos.sphere(position, Quat::IDENTITY, radius, color);
            }
            DrawGizmoEvent::Arrow {
                start_position,
                end_position,
                color,
                ..
            } => {
                gizmos.arrow(start_position, end_position, color);
            }
        }
    }
}
