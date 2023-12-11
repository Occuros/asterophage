use crate::building::building_systems::*;
use bevy::prelude::*;

use self::building_components::{Building, Extractor, BeltPiece, CompleteConveryorBelt};

pub mod building_components;
mod building_systems;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Building>()
        .register_type::<Extractor>()
        .register_type::<BeltPiece>()
        .register_type::<CompleteConveryorBelt>()
            .add_systems(Update, place_building_system)
            // .add_systems(Update, test_place_building_system)

        ;
    }
}
