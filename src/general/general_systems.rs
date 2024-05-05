use std::f32::consts::TAU;

use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::{SpatialQuery, SpatialQueryFilter};
use crate::building::building_components::*;
use crate::general::general_components::BuildingButton;
use crate::MainCamera;
use crate::player::player_components::GameCursor;
use crate::world_grid::world_gird_components::*;
use bevy_vector_shapes::prelude::*;


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn setup_menu(
    mut commands: Commands,
    camera_query: Query<Entity, With<MainCamera>>,
    asset_server: Res<AssetServer>,
)
{
    let main_camera = camera_query.get_single().unwrap();
    // root node
    commands
        .spawn((
            TargetCamera(main_camera),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            }))
            .with_children(|parent| {
                // left vertical fill (border)
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(100.),
                            border: UiRect::all(Val::Px(2.)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // left vertical fill (content)
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Column,
                                    // justify_content: JustifyContent::FlexStart,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Px(40.0),
                                        ..default()
                                    },
                                    ..default()
                                });
                                // text
                                parent.spawn((
                                    TextBundle::from_section(
                                        "Build Menu",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                            font_size: 20.0,
                                            ..default()
                                        },
                                    )
                                        .with_style(Style {
                                            margin: UiRect::all(Val::Px(5.)),
                                            ..default()
                                        }),
                                    // Because this is a distinct label widget and
                                    // not button/list item text, this is necessary
                                    // for accessibility to treat the text accordingly.
                                    Label,
                                ));
                                parent
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            width: Val::Px(50.),
                                            height: Val::Px(50.0),
                                            border: UiRect::all(Val::Px(1.0)),
                                            // horizontally center child text
                                            justify_content: JustifyContent::Center,
                                            // vertically center child text
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        border_color: BorderColor(Color::BLACK),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    })
                                    .insert(BuildingButton {
                                        building_type: BuildingType::Extractor,
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            "Extractor",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                                font_size: 10.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ));
                                    });

                                parent
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            width: Val::Px(50.),
                                            height: Val::Px(50.0),
                                            border: UiRect::all(Val::Px(1.0)),
                                            // horizontally center child text
                                            justify_content: JustifyContent::Center,
                                            // vertically center child text
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        border_color: BorderColor(Color::BLACK),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    })
                                    .insert(BuildingButton {
                                        building_type: BuildingType::ConveyorBelt,
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            "Belt",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                                font_size: 10.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ));
                                    });
                            });
                    });
            });
}

pub fn button_highlight_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn update_cursor_system(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera>, With<MainCamera>)>,
    mut game_cursor: ResMut<GameCursor>,
    spatial_query: SpatialQuery,
) {
    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_query.get_single().unwrap();
    game_cursor.ui_position = window.cursor_position();
    if let Some(cursor_position) = window.cursor_position() {
        let ray = camera.viewport_to_world(camera_transform, cursor_position);
        let filter = SpatialQueryFilter::default();
        if let Some(ray) = ray {
            if let Some(hit) = spatial_query.cast_ray(ray.origin, ray.direction, f32::MAX, true, filter) {
                let position = ray.origin + ray.direction * hit.time_of_impact;
                game_cursor.world_position = Some(position);
            }
        }
    }
}


pub fn building_ui_selection_system(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut interaction_query: Query<
        (
            &Interaction,
            &BuildingButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_cursor: ResMut<GameCursor>,
    world_grid: Res<WorldGrid>,
    mut shapes: ShapeCommands,
) {
    for (interaction, building_button) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        if *interaction == Interaction::Pressed {
            if game_cursor.preview_entity.is_some() {
                if let Some(entity_command) = commands.get_entity(game_cursor.preview_entity.unwrap()) {
                    entity_command.despawn_recursive();
                }
            }

            game_cursor.preview_entity = Building::spawn(
                building_button.building_type,
                game_cursor.world_position.unwrap_or_default(),
                Quat::default(),
                world_grid.grid_size,
                &mut commands,
                &mut asset_server,
                &mut shapes,
            );
        }
    }
}


pub fn move_building_preview_with_cursor_system(
    game_cursor: Res<GameCursor>,
    mut transform_q: Query<&mut Transform>,
    world_grid: Res<WorldGrid>,
) {
    if game_cursor.preview_entity.is_none() { return; }
    let cursor_grid_position = world_grid
        .get_grid_position_from_world_position(game_cursor.world_position.unwrap_or_default());
    let cursor_position = world_grid.grid_to_world(&cursor_grid_position);
    if let Ok(mut transform) = transform_q.get_mut(game_cursor.preview_entity.unwrap()) {
        transform.translation = cursor_position + Vec3::Y * 0.1;
    }
}

pub fn remove_preview_building_system(
    mut commands: Commands,
    mut game_cursor: ResMut<GameCursor>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_input.just_pressed(MouseButton::Right) { return; }
    if game_cursor.preview_entity.is_none() { return; }
    commands.entity(game_cursor.preview_entity.unwrap()).despawn_recursive();
    game_cursor.preview_entity = None;
}

pub fn rotate_preview_item_system(
    game_cursor: Res<GameCursor>,
    input: Res<ButtonInput<KeyCode>>,
    mut tranform_q: Query<&mut Transform>,
) {
    if !input.just_pressed(KeyCode::KeyR) { return; }
    if game_cursor.preview_entity.is_none() { return; }

    let mut transform = tranform_q.get_mut(game_cursor.preview_entity.unwrap()).unwrap();

    transform.rotate_y(-TAU * 0.25);
    info!("piece is rotated {:?}", transform.grid_rotation());
}

