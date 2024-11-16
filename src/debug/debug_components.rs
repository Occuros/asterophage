use bevy::prelude::*;

#[derive(Component, Default)]
pub struct DebugText;

#[derive(Event, Default)]
pub struct CursorDebugTextEvent {
    pub text: String,
}

#[derive(Component)]
pub struct CursorPositionDebug;
