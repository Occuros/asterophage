use bevy::prelude::*;

#[derive(Component, Default)]
pub struct DebugText;

#[derive(Event, Default)]
pub struct CursorDebugTextEvent {
    pub text: String,
}

#[derive(Component)]
pub struct CursorPositionDebug;

pub struct DebugSettings {
    pub draw_conveyors: bool,
}

#[derive(Component, Reflect, Default, Debug)]
pub struct DebugInfoPanel {
    pub selected_entity: Option<Entity>,
}
