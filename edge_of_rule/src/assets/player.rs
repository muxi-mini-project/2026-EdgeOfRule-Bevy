use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct PlayerAssets {
    pub texture: Handle<Image>,
}

pub fn load_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerAssets {
        texture: asset_server.load("images/player.png"),
    });
}
