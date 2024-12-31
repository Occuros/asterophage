use crate::building::building_components::{BeltElement, ConveyorSegment};
use crate::building::conveyor_belt::ConveyorBelt;
use crate::debug::debug_components::*;
use crate::general::general_components::GeneralAssets;
use crate::player::player_components::GameCursor;
use crate::world_grid::world_gird_components::*;
use crate::MainCamera;
use bevy::color::palettes::css::*;
use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;

pub fn debug_setup(
    mut commands: Commands,
    general_assets: Res<GeneralAssets>,
    camera_query: Query<Entity, With<MainCamera>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let main_camera = camera_query.get_single().unwrap();
    for (_, config, _) in config_store.iter_mut() {
        config.depth_bias = -1.;
    }
    let segment = ConveyorSegment::new(Vec3::new(0.0, 0.0, -1.5), Vec3::new(-3.25, 0.0, -1.5));
    let point = Vec3::new(-0.3258169, 0.0, -1.4999999);

    info!("is point on segment {}", segment.point_on_segment(point));

    commands.spawn((
        TargetCamera(main_camera),
        Text("hello\nbevy!".to_owned()),
        TextLayout::new(JustifyText::Left, LineBreak::NoWrap),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        TextFont {
            font: general_assets.default_font.clone(),
            font_size: 10.0,
            ..default()
        },
        DebugText,
        Name::new("Debug Text"),
    ));

    commands.spawn((
        TargetCamera(main_camera),
        TextLayout::new(JustifyText::Left, LineBreak::NoWrap),
        Node {
            position_type: PositionType::Absolute,
            align_content: AlignContent::FlexStart,
            align_items: AlignItems::FlexStart,
            flex_direction: FlexDirection::Column, // Stack children vertically
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            min_width: Val::Px(100.0),
            min_height: Val::Px(60.0),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(GRAY_900.into()),
        BorderRadius::all(Val::Px(5.0)),
        Name::new("Debug Info Panel"),
        DebugInfoPanel::default(),
    ));
}

pub fn cursor_position_debug_system(
    game_cursor: Res<GameCursor>,
    mut debug_text_event: EventWriter<CursorDebugTextEvent>,
    world_grid: Res<WorldGrid>,
) {
    let position = world_grid
        .grid_position_from_world_position(game_cursor.world_position.unwrap_or_default());

    let world_position = game_cursor.world_position.unwrap_or_default();
    debug_text_event.send(CursorDebugTextEvent {
        text: format!(
            "x:{} y:{}\n\
            x:{:.1?}z:{:.1?}",
            position.x, position.y, world_position.x, world_position.z,
        ),
    });
}

pub fn move_debug_text_system(
    window_query: Query<&Window>,
    mut debug_text_q: Query<&mut Node, With<DebugText>>,
) {
    let window = window_query.get_single().unwrap();
    if let Some(cursor_position) = window.cursor_position() {
        let mut node = debug_text_q.single_mut();
        node.left = Val::Px(cursor_position.x + 20.0);
        node.top = Val::Px(cursor_position.y + 20.0);
    }
}

pub fn change_debug_text_system(
    mut debug_text_event: EventReader<CursorDebugTextEvent>,
    mut debut_text_q: Query<&mut Text, With<DebugText>>,
) {
    if debut_text_q.get_single().is_err() {
        return;
    }
    let mut text = debut_text_q.single_mut();

    for event in debug_text_event.read() {
        text.0 = event.text.to_owned();
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
                shapes.color = RED.into();
                shapes.circle(0.15);
            }
            // shapes.rect(Vec2::splat(0.9 * world_grid.grid_size));

            for segment in &conveyor.segments {
                // shapes.transform = Transform::from_translation(Vec3::Y * 0.15);
                let offset = Vec3::Y * 0.15;
                shapes.color = PINK_600.into();
                shapes.thickness = 0.1;
                shapes.hollow = false;
                shapes.transform = Transform::from_translation(segment.start_position() + offset)
                    .with_rotation(Quat::from_rotation_x(TAU * 0.25));
                shapes.circle(0.05);

                shapes.color = if segment.is_connector {
                    GREEN_500.into()
                } else {
                    PURPLE.into()
                };

                shapes.transform = Transform::from_translation(segment.end_position() + offset)
                    .with_rotation(Quat::from_rotation_x(TAU * 0.25));

                shapes.circle(0.05);
                shapes.transform = Transform::from_translation(Vec3::Y * 0.12);
                shapes.alignment = Alignment::Billboard;
                shapes.thickness = 0.02;
                shapes.line(segment.start_position(), segment.end_position());

                // for item in &conveyor.items {
                //     shapes.transform.translation = item.position + Vec3::Y * 0.5;
                //     shapes.rect(Vec2::splat(0.1));
                // }
            }
        }
    }
}

pub fn hover_selection_system(
    game_cursor: Res<GameCursor>,
    world_grid: Res<WorldGrid>,
    mut debug_info_panel_q: Single<&mut DebugInfoPanel>,
) {
    let Some(world_position) = game_cursor.world_position else {
        return;
    };
    let grid_position = world_grid.grid_position_from_world_position(world_position);
    let Some(cell) = world_grid.cells.get(&grid_position) else {
        return;
    };
    let mut debug_info_panel = debug_info_panel_q.into_inner();
    match cell.surface_layer {
        SurfaceLayer::Empty => {
            debug_info_panel.selected_entity = None;
        }
        SurfaceLayer::Building { entity } => debug_info_panel.selected_entity = Some(entity),
        SurfaceLayer::Resource { entity } => {
            debug_info_panel.selected_entity = None;
        }
    }
}

pub fn debug_hover_system(
    mut commands: Commands,
    mut info_panel_q: Single<(Entity, &DebugInfoPanel)>,
    general_assets: Res<GeneralAssets>,
    belt_q: Query<&BeltElement>,
    conveyor_q: Query<&ConveyorBelt>,
    mut gizmos: Gizmos,
) {
    let (entity, info_panel) = info_panel_q.into_inner();

    commands.entity(entity).despawn_descendants();

    let Some(building_entity) = info_panel.selected_entity else {
        return;
    };

    if let Some(conveyor) = belt_q
        .get(building_entity)
        .ok()
        .and_then(|belt| belt.conveyor_belt)
        .and_then(|conveyor_entity| conveyor_q.get(conveyor_entity).ok())
    {
        let up = Vec3::Y * 0.1;
        for segment in &conveyor.segments {
            // let dir = segment.direction;
            // let perp = Vec3::new(-dir.z, 0.0, dir.x).normalize();
            // let offset = perp * 0.25;
            // gizmos.line(
            //     segment.start_position + up - offset,
            //     segment.end_position + up - offset,
            //     ORANGE_400,
            // );
            // gizmos.line(
            //     segment.start_position + up + offset,
            //     segment.end_position + up + offset,
            //     ORANGE_400,
            // );
            gizmos.rect(
                Isometry3d {
                    translation: (segment.start_position()
                        + segment.direction() * segment.length() * 0.5)
                        .into(),
                    rotation: Quat::from_rotation_x(TAU * 0.25),
                },
                segment.rect().size(),
                ORANGE_400,
            );
        }

        let up = Vec3::Y * 0.00;
        let segment_colors = [ORANGE_400, GREEN_400, BLUE_500, PURPLE_600];
        for item in &conveyor.items {
            gizmos.circle(
                Isometry3d::new(item.position + up, Quat::from_rotation_x(TAU * 0.25)),
                0.05,
                segment_colors[item.segment_index % segment_colors.len()],
            );
        }
        commands.entity(entity).with_children(|commands| {
            commands.spawn((
                Text("Conveyor".to_owned()),
                TextFont {
                    font: general_assets.default_font.clone(),
                    font_size: 12.0,
                    ..default()
                },
            ));
            commands.spawn((
                Text(format!("segments: {}", conveyor.segments.len())),
                TextFont {
                    font: general_assets.default_font.clone(),
                    font_size: 10.0,
                    ..default()
                },
            ));

            commands.spawn((
                Text(format!("items: {}", conveyor.items.len())),
                TextFont {
                    font: general_assets.default_font.clone(),
                    font_size: 10.0,
                    ..default()
                },
            ));

            for (i, item) in conveyor.items.iter().enumerate() {
                commands.spawn((
                    Text(format!(
                        "i: {:>3} s:{} - p:{:.2} - {} {}",
                        i,
                        item.segment_index,
                        item.segment_progress,
                        item.position,
                        item.item_entity
                    )),
                    TextFont {
                        font: general_assets.default_font.clone(),
                        font_size: 10.0,
                        ..default()
                    },
                ));
            }
        });
    }
    // if let Ok(belt) = belt_q.get(building_entity) {
    //     if let Some(conveyor_entity) = belt.conveyor_belt {
    //         if let Ok(conveyor) = conveyor_q.get(conveyor_entity) {
    //             commands.entity(entity).with_children(|commands| {
    //                 commands.spawn((
    //                     Text("Conveyor".to_owned()),
    //                     TextFont {
    //                         font: general_assets.default_font.clone(),
    //                         font_size: 12.0,
    //                         ..default()
    //                     },
    //                 ));
    //                 commands.spawn((
    //                     Text(format!("segments: {}", conveyor.belt_pieces.len())),
    //                     TextFont {
    //                         font: general_assets.default_font.clone(),
    //                         font_size: 10.0,
    //                         ..default()
    //                     },
    //                 ));
    //
    //                 commands.spawn((
    //                     Text(format!("items: {}", conveyor.items.len())),
    //                     TextFont {
    //                         font: general_assets.default_font.clone(),
    //                         font_size: 10.0,
    //                         ..default()
    //                     },
    //                 ));
    //             });
    //         }
    //     }
    // }
}
