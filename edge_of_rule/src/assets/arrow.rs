use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct ArrowAssets {
    pub up: Handle<Image>,
    pub mid: Handle<Image>,
    pub down: Handle<Image>,
}

pub fn load_arrow_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ArrowAssets {
        up: asset_server.load("images/HUD/arrow/0.png"),
        mid: asset_server.load("images/HUD/arrow/1.png"),
        down: asset_server.load("images/HUD/arrow/2.png"),
    });
}
