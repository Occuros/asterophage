use crate::ReflectComponent;
use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Component, Direction3d, Entity, Reflect};
use crate::building::building_components::{BeltItem, BeltPiece, Segment};
use crate::world_grid::world_gird_components::{GridPosition, WorldGrid};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ConveyorBelt {
    //belt pieces first is at the start, last at the end
    pub belt_pieces: Vec<BeltPiece>,
    pub items: Vec<BeltItem>,
    pub segments: Vec<Segment>,
    pub belt_speed: f32,
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

    pub fn spawn_new(commands: &mut Commands, belt_piece: BeltPiece) -> Entity {
        let conveyor_belt_entity = commands.spawn((
            ConveyorBelt {
                belt_pieces: vec![belt_piece],
                belt_speed: 0.50,
                ..default()
            },
            Name::new("Conveyor"),
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



    pub fn create_segments(&mut self, world_grid: &WorldGrid) {
        self.segments.clear();
        let segments = &mut self.segments;
        let belt_width_offset = 0.25; // Half of the belt width (0.5 / 2)

        // Initialize the first segment
        let mut current_segment = Segment::default();
        let mut previous_belt = None;
        let mut is_first= true;

        for belt in &self.belt_pieces {
            let belt_position = world_grid.grid_to_world(&belt.grid_position);

            // Set up the first segment start position if it's the first piece
            if previous_belt.is_none() {
                // Adjust the start position slightly backward to align with the belt piece's center
                current_segment.start_position = belt_position;

                if is_first {
                    current_segment.start_position  -= belt.grid_rotation.get_direction() * belt_width_offset;
                    is_first = false;
                }
                previous_belt = Some(belt);
                continue;
            }

            let previous_belt_position = world_grid.grid_to_world(&previous_belt.unwrap().grid_position);

            // Check if the direction has changed; if so, finalize the current segment
            if previous_belt.unwrap().grid_rotation != belt.grid_rotation {
                // Adjust the end position slightly forward to align with the belt piece's center
                current_segment.end_position = previous_belt_position + previous_belt.unwrap().grid_rotation.get_direction() * belt_width_offset * 2.0;
                current_segment.direction = (current_segment.end_position - current_segment.start_position).normalize();
                current_segment.length = (current_segment.end_position - current_segment.start_position).length();
                segments.push(current_segment);

                // Start a new segment
                current_segment = Segment::default();
                current_segment.start_position = belt_position
            } else {
                // Continue the current segment without changing direction
                current_segment.end_position = belt_position;
            }

            previous_belt = Some(belt);
        }

        // Finalize the last segment if there's an unfinished one
        if let Some(previous_belt) = previous_belt {
            // Adjust the end position of the last segment
            current_segment.end_position = world_grid.grid_to_world(&previous_belt.grid_position) + previous_belt.grid_rotation.get_direction() * belt_width_offset;
            current_segment.direction = (current_segment.end_position - current_segment.start_position).normalize();
            current_segment.length = (current_segment.end_position - current_segment.start_position).length();
            segments.push(current_segment);
        }

        let segments = &self.segments;

        let mut item_updates = vec![];
        for item in &self.items {
            let position = item.position;
            let segment_index = self.get_segment_index_for_position(position).unwrap();
            let segment: &Segment = &self.segments[segment_index];
            let progress = segment.progress_for_point(item.position);
            item_updates.push((segment_index, progress));
        }

        for i in 0..item_updates.len() {
            self.items[i].segment_index = item_updates[i].0;
            self.items[i].segment_progress = item_updates[i].1;
        }
    }



    pub fn get_segment_index_for_position(&self, position: Vec3) -> Option<usize> {
        for i in 0..self.segments.len() {
            let segment = &self.segments[i];
            if segment.point_on_segment(position) {
                return Some(i);
            }
        }
        None
    }

    pub fn has_space_at_position(&self, position: Vec3, item_size: f32) -> bool {
        let Some(i) = self.get_segment_index_for_position(position) else {return false};
        for item in self.items.iter() {
            if item.segment_index < i {continue};
            if item.segment_index > i {return true};
            if item.position.distance_squared(position) < item_size {return false};
        }

        true
    }
}