use bevy::prelude::*;

use crate::{
    assets::player::PlayerAssets,
    entities::player::{Player, spawn_player},
};

pub fn spawn(commands: Commands, asset: Res<PlayerAssets>) {
    spawn_player(commands, asset, Transform::from_xyz(-50.0, 20.0, 0.0));
}

pub fn despawn(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player in &players {
        commands.entity(player).despawn();
    }
}
