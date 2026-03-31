use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct LevelsImageAssets {
    pub day1_scene1_background: Handle<Image>,
    pub day1_scene2_background: Handle<Image>,
    pub day1_scene3_background: Handle<Image>,
    pub day1_scene4_background: Handle<Image>,
    pub day1_scene3_fog: Handle<Image>,
    pub day2_scene1_background: Handle<Image>,
}

pub fn load_levels_image_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LevelsImageAssets {
        day1_scene1_background: asset_server.load("images/levels/day1/scene1_background.png"),
        day1_scene2_background: asset_server.load("images/levels/day1/scene2_background.png"),
        day1_scene3_background: asset_server.load("images/levels/day1/scene3_background.png"),
        day1_scene4_background: asset_server.load("images/levels/day1/scene4_background.png"),
        day1_scene3_fog: asset_server.load("images/levels/day1/scene3_fog.png"),
        day2_scene1_background: asset_server.load("images/levels/day2/scene1_background.png"),
    });
}
