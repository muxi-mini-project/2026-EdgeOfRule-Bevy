use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct ElevatorAssets {
    pub up: Handle<Image>,
    pub stop: Handle<Image>,
    pub down: Handle<Image>,
}

pub fn load_elevator_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ElevatorAssets {
        up: asset_server.load("images/animations/elevator/up.png"),
        stop: asset_server.load("images/animations/elevator/stop.png"),
        down: asset_server.load("images/animations/elevator/down.png"),
    });
}

