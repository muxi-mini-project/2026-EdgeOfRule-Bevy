use crate::assets::player::PlayerAssets;
use crate::entities::general::InGameEntity;
use bevy::prelude::*;

#[derive(Component)]
struct Player;

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
