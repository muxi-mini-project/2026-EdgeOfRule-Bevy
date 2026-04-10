use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct LiftAssets {
    pub broken: Handle<Image>,
    pub fixed_0: Handle<Image>,
    pub fixed_1: Handle<Image>,
    pub fixed_2: Handle<Image>,
    pub fixed_3: Handle<Image>,
}

pub fn load_lift_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LiftAssets {
        broken: asset_server.load("images/animations/lift/broken.png"),
        fixed_0: asset_server.load("images/animations/lift/fixed_0.png"),
        fixed_1: asset_server.load("images/animations/lift/fixed_1.png"),
        fixed_2: asset_server.load("images/animations/lift/fixed_2.png"),
        fixed_3: asset_server.load("images/animations/lift/fixed_3.png"),
    });
}
