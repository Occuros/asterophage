use bevy::prelude::*;
use crate::building::building_components::BuildingTypes;

#[derive(Component, Default, Reflect)]
pub struct BuildingButton {
    pub building_type: BuildingTypes
}