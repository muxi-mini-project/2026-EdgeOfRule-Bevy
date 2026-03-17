use bevy::prelude::*;

use crate::{
    assets::player::PlayerAssets,
    entities::player::{Player, SpawnPoint, spawn_player},
};

pub fn spawn(commands: Commands, asset: Res<PlayerAssets>, spawn_point: Res<SpawnPoint>) {
    spawn_player(commands, asset, spawn_point.0);
}

pub fn despawn(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player in &players {
        commands.entity(player).despawn();
    }
}
