use crate::building::building_components::{BeltItem, BeltPiece, ConveyorSegment};
use crate::world_grid::world_gird_components::*;
use crate::ReflectComponent;
use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::utils::info;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct ConveyorBelt {
    //belt pieces first is at the start, last at the end
    pub belt_pieces: Vec<BeltPiece>,
    pub items: Vec<BeltItem>,
    pub segments: Vec<ConveyorSegment>,
    pub belt_speed: f32,
    pub connected_conveyor_belt: Option<Entity>,
}

#[derive(Event)]
pub struct ConveyorSegmentsChanged;

#[derive(Event)]
pub struct ItemReachedOtherBeltTrigger {
    pub belt_item: BeltItem,
    pub next_conveyor: Entity,
}

impl ConveyorBelt {
    pub fn start_position(&self) -> GridPosition {
        match self.belt_pieces.first() {
            None => GridPosition::default(),
            Some(belt) => belt.grid_position,
        }
    }

    pub fn end_position(&self) -> GridPosition {
        match self.belt_pieces.last() {
            None => GridPosition::default(),
            Some(belt) => belt.grid_position,
        }
    }

    pub fn spawn_new(commands: &mut Commands, belt_piece: BeltPiece) -> Entity {
        let conveyor_belt_entity = commands
            .spawn((
                ConveyorBelt {
                    belt_pieces: vec![belt_piece],
                    belt_speed: 2.50,
                    ..default()
                },
                Name::new("Conveyor"),
            ))
            .id();
        conveyor_belt_entity
    }

    pub fn get_connecting_positions_from_start(&self) -> Vec<GridPosition> {
        let Some(start_piece) = self.belt_pieces.first() else {
            return vec![];
        };
        vec![
            self.start_position()
                .get_relative_left(start_piece.grid_rotation),
            self.start_position()
                .get_relative_back(start_piece.grid_rotation),
            self.start_position()
                .get_relative_right(start_piece.grid_rotation),
        ]
    }

    pub fn get_connecting_positions_from_end(&self) -> Vec<GridPosition> {
        let Some(last_piece) = self.belt_pieces.last() else {
            return vec![];
        };
        vec![
            self.end_position()
                .get_relative_left(last_piece.grid_rotation),
            self.end_position()
                .get_relative_forward(last_piece.grid_rotation),
            self.end_position()
                .get_relative_right(last_piece.grid_rotation),
        ]
    }

    pub fn can_connect_to_start_piece(&self, other_piece: &BeltPiece) -> bool {
        let Some(start_piece) = self.belt_pieces.first() else {
            return false;
        };
        if !self
            .get_connecting_positions_from_start()
            .contains(&other_piece.grid_position)
        {
            return false;
        }
        other_piece.relative_forward_position() == start_piece.grid_position
    }

    pub fn end_piece_can_connect_to(&self, other_piece: &BeltPiece) -> bool {
        let Some(end_piece) = self.belt_pieces.last() else {
            return false;
        };
        if !self
            .get_connecting_positions_from_end()
            .contains(&other_piece.grid_position)
        {
            return false;
        }

        end_piece.relative_forward_position() == other_piece.grid_position
    }

    ///Inserts item and the correct position, will not work if there is no space on the belt
    pub fn insert_item(&mut self, item: &BeltItem) {
        let mut item = item.clone();
        let index = self
            .get_segment_index_for_position(item.position)
            .expect(&format!(
                "Somehow item not on any segment {} - {:?}",
                item.position, self.segments
            ));
        let progress = self.segments[index].progress_for_point(item.position);
        item.segment_index = index;
        item.segment_progress = progress;

        let mut index_to_insert = self.items.len();

        for (i, existing_item) in self.items.iter().enumerate() {
            if existing_item.segment_index > index {
                continue;
            }
            if existing_item.segment_index < index {
                index_to_insert = i;
                break;
            }
            if existing_item.segment_index == index {
                if existing_item.segment_progress >= item.segment_progress {
                    continue;
                } else {
                    index_to_insert = i;
                    break;
                }
            }
        }

        self.items.insert(index_to_insert, item);
    }

    pub fn remove_item(&mut self, belt_item: &BeltItem) {
        self.items
            .retain(|item| item.item_entity != belt_item.item_entity);
    }

    #[allow(dead_code)]
    pub fn get_belt_piece_at_position(&self, grid_position: &GridPosition) -> Option<&BeltPiece> {
        for belt_piece in &self.belt_pieces {
            if &belt_piece.grid_position == grid_position {
                return Some(belt_piece);
            }
        }
        None
    }

    pub fn get_segment_index_for_position(&self, position: Vec3) -> Option<usize> {
        for i in 0..self.segments.len() {
            let segment = &self.segments[i];
            if segment.is_connector {
                continue;
            };
            if segment.point_on_segment(position) {
                return Some(i);
            }
        }
        None
    }

    pub fn has_space_at_position(&self, position: Vec3, item_size: f32) -> bool {
        let Some(i) = self.get_segment_index_for_position(position) else {
            info!("no index found for position {}", position);
            return false;
        };
        for item in self.items.iter() {
            if item.segment_index < i {
                continue;
            };
            if item.segment_index > i {
                return true;
            };

            if item.position.distance_squared(position) < (item_size * item_size) {
                return false;
            };
        }

        true
    }
}
