use bevy::prelude::*;
use bevy::ui::Val::Px;
use crate::debug::debug_components::*;
use crate::player::player_components::GameCursor;
use crate::world_grid::world_gird_components::WorldGrid;

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