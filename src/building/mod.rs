use crate::building::building_systems::*;
use bevy::prelude::*;
use crate::building::building_components::{BuildingPlacedEvent, BuildingRemovedEvent, ConveyorPlacedEvent, Inserter};

use self::building_components::{Building, Extractor, BeltElement, ConveyorBelt};

pub mod building_components;
mod building_systems;

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
            .add_systems(Update, respond_to_conveyor_belt_placement_event.after(place_building_system))
            .add_systems(Update, handle_conveyor_placement_system.after(respond_to_conveyor_belt_placement_event))
            .add_systems(Update, remove_building_system)
            .add_systems(Update, respond_to_belt_element_removal.after(remove_building_system))
            .add_systems(Update, extract_resources_system)
            // .add_systems(Update, belt_system)
            .add_systems(Update, spatial_belt_system)

            .add_systems(Update, inserter_animation_system)
            .add_systems(Update, inserter_system)

            .add_systems(PostUpdate, destroy_building_system)

        // .add_systems(Update, test_place_building_system)

        ;
    }
}
