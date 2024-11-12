use crate::general::Pastel;
use crate::world_grid::world_gird_components::*;
use bevy::prelude::TimerMode::Repeating;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;
use std::time::Duration;
use bevy::ecs::system::{EntityCommand, EntityCommands};
use bevy::reflect::List;

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
                position, rotation, size, commands, shapes, asset_server,
            )),
            BuildingType::InserterType => Some(Inserter::spawn(position, rotation, size, commands, asset_server))
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
    pub item: Option<Entity>,
    pub item_reached_center: bool,
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
                    Quat::from_rotation_y(TAU * 0.25) * Quat::from_rotation_x(TAU * 0.25) * Quat::from_rotation_z(TAU * 0.25),
                )
                    .with_translation(Vec3::Y * 0.25);
                shapes.thickness = 0.01;
                shapes.color = Color::YELLOW.pastel();
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
                Inserter {
                    ..default()
                },
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

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ConveyorBelt {
    //belt pieces first is at the start, last at the end
    pub belt_pieces: Vec<BeltPiece>,
    pub items: Vec<BeltItem>,
    // pub segments: Vec<Segment>
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ConveyorSegments {
    pub segments: Vec<Segment>,
}

impl ConveyorSegments {
    pub fn get_segment_index_for_position(&self, position: Vec3) -> Option<usize> {
        for i in 0..self.segments.len() {
            let segment = &self.segments[i];
            if segment.point_on_segment(position) {
                return Some(i);
            }
        }
        None
    }
}


#[derive(Default, Debug, Reflect, Clone)]
pub struct Segment {
    pub start_position: Vec3,
    pub end_position: Vec3,
    pub direction: Vec3,
    pub length: f32,
    pub max_progress: f32,
    pub is_blocked: bool,
}
impl Segment {
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
}

#[derive(Reflect, Debug)]
pub struct BeltItem {
    pub item_entity: Entity,
    pub position: Vec3,
    pub segment_progress: f32,
    pub segment_index: usize,
}

impl ConveyorBelt {
    pub fn start_position(&self) -> GridPosition {
        match self.belt_pieces.first() {
            None => GridPosition::default(),
            Some(belt) => belt.grid_position
        }
    }

    pub fn end_position(&self) -> GridPosition {
        match self.belt_pieces.last() {
            None => GridPosition::default(),
            Some(belt) => belt.grid_position
        }
    }

    pub fn try_add_item(item_entity: Entity, position: Vec3) -> bool {
        false
    }

    pub fn spawn_new(commands: &mut Commands, belt_piece: BeltPiece) -> Entity {
        let conveyor_belt_entity = commands.spawn((
            ConveyorBelt {
                belt_pieces: vec![belt_piece],
                ..default()
            },
            Name::new("Conveyor"),
            ConveyorSegments::default(),
        )).id();
        conveyor_belt_entity
    }

    pub fn get_connecting_positions_from_start(&self) -> Vec<GridPosition> {
        let Some(start_piece) = self.belt_pieces.first() else { return vec![]; };
        vec![
            self.start_position().get_relative_left(start_piece.grid_rotation),
            self.start_position().get_relative_back(start_piece.grid_rotation),
            self.start_position().get_relative_right(start_piece.grid_rotation),
        ]
    }

    pub fn get_connecting_positions_from_end(&self) -> Vec<GridPosition> {
        let Some(last_piece) = self.belt_pieces.last() else { return vec![]; };
        vec![
            self.end_position().get_relative_left(last_piece.grid_rotation),
            self.end_position().get_relative_forward(last_piece.grid_rotation),
            self.end_position().get_relative_right(last_piece.grid_rotation),
        ]
    }


    pub fn can_connect_to_start_piece(&self, other_piece: &BeltPiece) -> bool {
        let Some(start_piece) = self.belt_pieces.first() else { return false; };
        if !self.get_connecting_positions_from_start().contains(&other_piece.grid_position) { return false; }
        start_piece
            .grid_rotation
            .difference(other_piece.grid_rotation)
            < 2
    }

    pub fn end_piece_can_connect_to(&self, other_piece: &BeltPiece) -> bool {
        let Some(end_piece) = self.belt_pieces.last() else { return false; };
        if !self.get_connecting_positions_from_end().contains(&other_piece.grid_position) { return false; }
        end_piece
            .grid_rotation
            .difference(other_piece.grid_rotation)
            < 2
    }

    pub fn create_segments(&self, world_grid: &WorldGrid) -> Vec<Segment> {
        let mut segments = vec![];
        println!("creating segments");
        let mut current_segment = Segment::default();
        let mut previous_belt = None;
        for belt in &self.belt_pieces {
            if previous_belt.is_none() {
                previous_belt = Some(belt);
                current_segment.start_position = world_grid.grid_to_world(&belt.grid_position);
                continue;
            }

            if previous_belt.unwrap().grid_rotation == belt.grid_rotation {
                previous_belt = Some(belt);
                continue;
            }

            current_segment.end_position = world_grid.grid_to_world(&previous_belt.unwrap().grid_position);
            let direction = (current_segment.end_position - current_segment.start_position).normalize();
            current_segment.start_position -= direction * 0.5;
            current_segment.end_position += direction * 0.5;
            current_segment.direction = direction;
            current_segment.length = (current_segment.end_position - current_segment.start_position).length();
            println!("added segments {:?}", current_segment);

            segments.push(current_segment);
            previous_belt = None;
            current_segment = Segment::default();
        }

        if let Some(previous_belt) = previous_belt {
            current_segment.end_position = world_grid.grid_to_world(&previous_belt.grid_position);
            let direction = (current_segment.end_position - current_segment.start_position).normalize();
            current_segment.start_position -= direction * 0.5;
            current_segment.end_position += direction * 0.20;
            current_segment.direction = direction;
            current_segment.length = (current_segment.end_position - current_segment.start_position).length();
            println!("added segments {:?}", current_segment);
            segments.push(current_segment);
        }
        segments
    }


}

#[derive(Reflect, Clone, Copy, Debug)]
pub struct BeltPiece {
    pub entity: Entity,
    pub grid_rotation: GridRotation,
    pub grid_position: GridPosition,
}
