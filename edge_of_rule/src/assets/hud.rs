use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct HudImageAssets {
    pub blood: Handle<Image>,
    pub blood_bar: Handle<Image>,
    pub blue: Handle<Image>,
    pub blue_bar: Handle<Image>,
}

pub fn load_hud_image_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(HudImageAssets {
        blood: asset_server.load("images/HUD/blood.png"),
        blood_bar: asset_server.load("images/HUD/blood_bar.png"),
        blue: asset_server.load("images/HUD/blue.png"),
        blue_bar: asset_server.load("images/HUD/blue_bar.png"),
    });
}
