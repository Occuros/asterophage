use self::building_components::*;
use crate::building::building_components::{
    BuildingPlacedEvent, BuildingRemovedEvent, ConveyorPlacedEvent, Inserter,
};
use crate::building::building_systems::*;
use crate::building::conveyor_belt::ConveyorBelt;
use crate::building::conveyor_belt_systems::{conveyor_system, segments_changed};
use bevy::prelude::*;

pub mod building_components;
mod building_systems;
pub mod conveyor_belt;
mod conveyor_belt_systems;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildingPlacedEvent>()
            .add_event::<ConveyorPlacedEvent>()
            .add_event::<BuildingRemovedEvent>()
            .register_type::<Building>()
            .register_type::<Extractor>()
            .register_type::<BeltElement>()
            .register_type::<ConveyorBelt>()
            .register_type::<Inserter>()
            .add_systems(Update, place_building_system)
            .add_systems(PostUpdate, respond_to_conveyor_belt_placement_event)
            .add_systems(PostUpdate, handle_conveyor_placement_system.after(respond_to_conveyor_belt_placement_event))
            .add_systems(Update, remove_building_system)
            .add_systems(Update, respond_to_belt_element_removal.after(remove_building_system))
            .add_systems(Update, extract_resources_system)
            .add_systems(Update, inserter_animation_system)
            // .add_systems(Update, inserter_system)
            .add_systems(PostUpdate, conveyor_system)

            .add_systems(PostUpdate, destroy_building_system)
            .add_observer(segments_changed)

        // .add_systems(Update, test_place_building_system)

        ;
    }
}

// #[derive(QueryData)]
// #[query_data(mutable)]
// pub struct ConveyorQuery {
//     conveyor_belt: &'static mut ConveyorBelt
// }
