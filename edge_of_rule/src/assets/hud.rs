use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct HudImageAssets {
    pub blood: Handle<Image>,
    pub blood_bar: Handle<Image>,
    pub blue: Handle<Image>,
    pub blue_bar: Handle<Image>,
    pub key_icon: Handle<Image>,
    pub screw_icon: Handle<Image>,
    pub controller_icon: Handle<Image>,
    pub cover_icon: Handle<Image>,
}

pub fn load_hud_image_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(HudImageAssets {
        blood: asset_server.load("images/HUD/blood.png"),
        blood_bar: asset_server.load("images/HUD/blood_bar.png"),
        blue: asset_server.load("images/HUD/blue.png"),
        blue_bar: asset_server.load("images/HUD/blue_bar.png"),
        key_icon: asset_server.load("images/animations/key.png"),
        screw_icon: asset_server.load("images/animations/screw.png"),
        controller_icon: asset_server.load("images/animations/controller.png"),
        cover_icon: asset_server.load("images/animations/key.png"),
    });
}
