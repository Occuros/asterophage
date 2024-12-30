use crate::world_grid::world_gird_components::*;
use bevy::color::palettes::css::YELLOW;
use bevy::prelude::TimerMode::Repeating;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Default, Reflect, Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum BuildingType {
    #[default]
    None,
    Extractor,
    ConveyorBelt,
    InserterType,
}

#[derive(Default, Reflect, Component)]
pub struct Preview {}

#[derive(Default, Reflect, Component)]
pub struct Active {}

#[derive(Event)]
pub struct BuildingPlacedEvent {
    pub building_type: BuildingType,
    pub grid_position: GridPosition,
    pub grid_rotation: GridRotation,
    pub entity: Entity,
}

#[derive(Event)]
pub struct ConveyorPlacedEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct BuildingRemovedEvent {
    pub building_entity: Entity,
    pub grid_position: GridPosition,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Building {
    pub building_type: BuildingType,
}

impl Building {
    pub fn spawn(
        building_type: BuildingType,
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
        shapes: &mut ShapeCommands,
    ) -> Option<Entity> {
        match building_type {
            BuildingType::None => None,
            BuildingType::Extractor => Some(Extractor::spawn(
                position,
                rotation,
                size,
                commands,
                asset_server,
            )),
            BuildingType::ConveyorBelt => Some(BeltElement::spawn(
                position,
                rotation,
                size,
                commands,
                shapes,
                asset_server,
            )),
            BuildingType::InserterType => Some(Inserter::spawn(
                position,
                rotation,
                size,
                commands,
                asset_server,
            )),
        }
    }
}

#[derive(Component, Default, Reflect, Debug)]
pub struct RequiresGround {
    pub allowed_ground: Vec<GroundLayerType>,
}

#[derive(Component, Default, Reflect)]
pub struct Extractor {
    pub timer: Timer,
}

impl Extractor {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
    ) -> Entity {
        let model = asset_server.load("models/extractor.glb#Scene0");
        commands
            .spawn((
                SceneRoot(model),
                Transform::from_translation(position)
                    .with_rotation(rotation)
                    .with_scale(Vec3::splat(size)),
                Building {
                    building_type: BuildingType::Extractor,
                },
                Extractor {
                    timer: Timer::new(Duration::from_secs_f32(3.0), Repeating),
                },
                RequiresGround {
                    allowed_ground: vec![
                        GroundLayerType::BloodResource,
                        GroundLayerType::BlackBileResource,
                        GroundLayerType::PhlegmResource,
                        GroundLayerType::YellowBileResource,
                    ],
                },
            ))
            .id()
    }
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct BeltElement {
    pub speed: f32,
    pub conveyor_belt: Option<Entity>,
    // pub item: Option<Entity>,
    // pub item_reached_center: bool,
}

impl BeltElement {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        shapes: &mut ShapeCommands,
        asset_server: &mut AssetServer,
    ) -> Entity {
        let model = asset_server.load("models/conveyor-bars-stripe.glb#Scene0");

        let entity = commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(position)
                        .with_rotation(rotation)
                        .with_scale(Vec3::splat(size)),
                    ..default()
                },
                BeltElement {
                    conveyor_belt: None,
                    speed: 1.0,
                    ..default()
                },
                Building {
                    building_type: BuildingType::ConveyorBelt,
                },
                Name::new("Belt Piece"),
            ))
            .with_children(|parent| {
                parent.spawn((
                    SceneRoot(model),
                    Transform {
                        translation: Vec3::Y * -0.05,
                        rotation: Quat::from_rotation_y(TAU * 0.25),
                        ..default()
                    },
                ));
            })
            .with_shape_children(&shapes.config(), |shapes| {
                shapes.hollow = true;
                shapes.transform = Transform::from_rotation(
                    Quat::from_rotation_y(TAU * 0.25)
                        * Quat::from_rotation_x(TAU * 0.25)
                        * Quat::from_rotation_z(TAU * 0.25),
                )
                .with_translation(Vec3::Y * 0.25);
                shapes.thickness = 0.01;
                shapes.color = YELLOW.into();
                shapes.ngon(3.0, 0.2);
                shapes.translate(Vec3::Y * -0.15);
                shapes.rect(Vec2::new(0.1, 0.3));
            })
            .id();
        entity
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Inserter {
    pub item: Option<Entity>,
    pub rotation_spot: Option<Entity>,
    pub target_reached: bool,
}

impl Inserter {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
        // shapes: &mut ShapeCommands,
    ) -> Entity {
        let model = asset_server.load("models/robot-arm-a.glb#Scene0");
        let entity = commands
            .spawn((
                SceneRoot(model),
                Transform {
                    translation: position,
                    rotation,
                    scale: Vec3::splat(size),
                },
                Inserter { ..default() },
                Building {
                    building_type: BuildingType::InserterType,
                },
                Name::new("Inserter"),
            ))
            .id();
        entity
    }
}

#[derive(Debug, Reflect, Clone)]
pub struct ConveyorSegment {
    pub start_position: Vec3,
    pub end_position: Vec3,
    pub direction: Dir3,
    pub length: f32,
    pub is_connector: bool,
}

impl Default for ConveyorSegment {
    fn default() -> Self {
        let line = Vec3::X - Vec3::ZERO;

        Self {
            start_position: Vec3::ZERO,
            end_position: Vec3::X,
            length: line.length(),
            direction: Dir3::new(line).unwrap(),
            is_connector: false,
        }
    }
}

impl ConveyorSegment {
    pub fn new(start_position: Vec3, end_position: Vec3) -> Self {
        let line = end_position - start_position;
        Self {
            start_position,
            end_position,
            direction: Dir3::new(line).unwrap(),
            length: line.length(),
            ..default()
        }
    }
    pub fn point_on_segment(&self, point: Vec3) -> bool {
        let start = self.start_position;
        let end = self.end_position;
        let width = 0.25;
        // Vector from start to end
        let ab = end - start;
        // Length of the segment
        let ab_length = ab.length();
        // Unit vector in the direction of ab
        let ab_unit = ab / ab_length;

        // Perpendicular vector to ab in the XZ plane
        let perp = Vec3::new(-ab_unit.z, 0.0, ab_unit.x);

        // Half-width vector
        let half_width_vector = perp * (width / 2.0);

        // Define the four corners of the rectangle
        let corner1 = start + half_width_vector;
        let corner2 = start - half_width_vector;
        let corner3 = end + half_width_vector;
        let corner4 = end - half_width_vector;

        // Vectors from point to each corner
        let ap = point - corner1;
        let bp = point - corner2;
        let cp = point - corner3;
        let dp = point - corner4;

        // Calculate cross products to determine if point is on the correct side of each edge
        let cross1 = ab_unit.cross(ap);
        let cross2 = ab_unit.cross(bp);
        let cross3 = ab_unit.cross(cp);
        let cross4 = ab_unit.cross(dp);

        // Check if point is within the parallel lines defined by the rectangle's width
        let within_width = cross1.y * cross2.y <= 0.0 && cross3.y * cross4.y <= 0.0;

        // Check if point is within the length of the rectangle
        let dot1 = ab_unit.dot(ap);
        let dot2 = ab_unit.dot(bp);
        let within_length = dot1 >= 0.0 && dot1 <= ab_length && dot2 >= 0.0 && dot2 <= ab_length;

        within_width && within_length
    }

    pub fn progress_for_point(&self, point: Vec3) -> f32 {
        let segment_vector = self.end_position - self.start_position;
        let point_vector = point - self.start_position;

        let projection = point_vector.dot(segment_vector) / segment_vector.length_squared();

        projection.clamp(0.0, 1.0)
    }

    pub fn position_for_progress(&self, progress: f32) -> Vec3 {
        self.start_position.lerp(self.end_position, progress)
    }
}

#[derive(Reflect, Debug, Clone)]
pub struct BeltItem {
    pub item_entity: Entity,
    pub position: Vec3,
    pub segment_progress: f32,
    pub segment_index: usize,
}

#[derive(Reflect, Clone, Copy, Debug)]
pub struct BeltPiece {
    pub entity: Entity,
    pub grid_rotation: GridRotation,
    pub grid_position: GridPosition,
}

impl BeltPiece {
    pub fn relative_forward_position(&self) -> GridPosition {
        self.grid_position.get_relative_forward(self.grid_rotation)
    }
    #[allow(dead_code)]
    pub fn relative_back_position(&self) -> GridPosition {
        self.grid_position.get_relative_back(self.grid_rotation)
    }

    #[allow(dead_code)]
    pub fn relative_left_position(&self) -> GridPosition {
        self.grid_position.get_relative_left(self.grid_rotation)
    }

    #[allow(dead_code)]
    pub fn relative_right_position(&self) -> GridPosition {
        self.grid_position.get_relative_right(self.grid_rotation)
    }
}
