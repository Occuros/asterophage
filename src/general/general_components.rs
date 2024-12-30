use crate::building::building_components::BuildingType;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct BuildingButton {
    pub building_type: BuildingType,
}

#[derive(Resource)]
pub struct GeneralAssets {
    pub default_font: Handle<Font>,
    pub default_font_size: f32,
}

impl FromWorld for GeneralAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let default_font = asset_server.load("fonts/FiraMono-Medium.ttf");
        GeneralAssets {
            default_font,
            default_font_size: 10.0,
        }
    }
}
