use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct PlayerAssets {
    texture: Handle<Image>,
}

pub fn load_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerAssets {
        texture: asset_server.load("images/player.png"),
    });
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct InGameEntity;

pub fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: player_assets.texture.clone(),
            transform: Transform::from_xyz(50.0, -50.0, 0.0),
            ..Default::default()
        },
        Player,
        InGameEntity,
    ));
}
