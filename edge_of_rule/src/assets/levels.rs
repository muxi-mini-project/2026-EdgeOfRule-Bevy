use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct LevelsImageAssets {
    pub day1_scene1_background: Handle<Image>,
}

pub fn load_levels_image_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LevelsImageAssets {
        day1_scene1_background: asset_server.load("images/levels/day1/scene1_background.png"),
    });
}
