use crate::world_grid::world_gird_components::*;
use bevy::color::palettes::css::YELLOW;
use bevy::prelude::TimerMode::Repeating;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Default, Reflect, Clone, Copy, PartialEq)]
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
                SceneBundle {
                    scene: model,
                    transform: Transform::from_translation(position)
                        .with_rotation(rotation)
                        .with_scale(Vec3::splat(size)),
                    ..default()
                },
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

#[derive(Component, Default, Reflect)]
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
                parent.spawn(SceneBundle {
                    scene: model,
                    transform: Transform::from_translation(Vec3::Y * -0.05)
                        .with_rotation(Quat::from_rotation_y(TAU * 0.25)),
                    ..default()
                });
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
                SceneBundle {
                    scene: model,
                    transform: Transform::from_translation(position)
                        .with_rotation(rotation)
                        .with_scale(Vec3::splat(size)),
                    ..default()
                },
                Inserter { ..default() },
                Building {
                    building_type: BuildingType::InserterType,
                },
                Name::new("Inserter"),
            ))
            // .with_shape_children(&shapes.config(), |shapes| {
            //     shapes.hollow = true;
            //     shapes.transform = Transform::from_rotation(
            //         Quat::from_rotation_y(TAU * 0.25) * Quat::from_rotation_x(TAU * 0.25) * Quat::from_rotation_z(TAU * 0.25),
            //     )
            //         .with_translation(Vec3::Y * 0.01);
            //     shapes.thickness = 0.01;
            //     shapes.color = Color::YELLOW.pastel();
            //     shapes.ngon(3.0, 0.2);
            //     shapes.translate(Vec3::Y * -0.15);
            //     shapes.rect(Vec2::new(0.1, 0.3));
            // })
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
        let a = self.start_position;
        let b = self.end_position;
        let ab = a - b;
        let ap = point - a;

        // Calculate the cross product to check collinearity
        let cross = ab.cross(ap);

        // Check if the cross product is close to zero (collinear)
        if cross.length_squared() > f32::EPSILON {
            return false;
        }

        // Check if the point is within the bounds of the segment
        let within_x_bounds = (point.x >= a.x.min(b.x)) && (point.x <= a.x.max(b.x));
        let within_y_bounds = (point.y >= a.y.min(b.y)) && (point.y <= a.y.max(b.y));
        let within_z_bounds = (point.z >= a.z.min(b.z)) && (point.z <= a.z.max(b.z));

        within_x_bounds && within_y_bounds && within_z_bounds
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
    pub fn relative_back_position(&self) -> GridPosition {
        self.grid_position.get_relative_back(self.grid_rotation)
    }

    pub fn relative_left_position(&self) -> GridPosition {
        self.grid_position.get_relative_left(self.grid_rotation)
    }

    pub fn relative_right_position(&self) -> GridPosition {
        self.grid_position.get_relative_right(self.grid_rotation)
    }
}
