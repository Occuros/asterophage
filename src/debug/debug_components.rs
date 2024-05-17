use bevy::prelude::*;
use crate::building::building_components::BuildingType;
use crate::world_grid::world_gird_components::{GridPosition, GridRotation};

#[derive(Component, Default)]
pub struct DebugText;

#[derive(Event, Default)]
pub struct CursorDebugTextEvent {
    pub text: String,
}


#[derive(Event, Clone)]
pub enum DrawGizmoEvent {
    Sphere {
        position: Vec3,
        radius: f32,
        color: Color,
        timer: Timer,
    },
    Arrow {
        start_position: Vec3,
        end_position: Vec3,
        color: Color,
        timer: Timer,
    },
}
