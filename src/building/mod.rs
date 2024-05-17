use crate::building::building_systems::*;
use bevy::prelude::*;
use crate::building::belt_system::*;
use crate::building::black_hole_system::black_hole_system;
use crate::building::building_components::*;
use crate::world_grid::components::yellow_bile::YellowBileItem;

pub mod building_components;
mod building_systems;
mod belt_system;
mod black_hole_system;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildingPlacedEvent>()
            .add_event::<ConveyorPlacedEvent>()
            .add_event::<BuildingRemovedEvent>()
            .register_type::<Building>()
            .register_type::<Extractor>()
            .register_type::<YellowBileItem>()
            .register_type::<BeltElement>()
            .register_type::<ConveyorBelt>()
            .register_type::<Inserter>()
            .register_type::<BlackHole>()
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
            .add_systems(Update, black_hole_system)

            .add_systems(PostUpdate, destroy_building_system)

        // .add_systems(Update, test_place_building_system)

        ;
    }
}
