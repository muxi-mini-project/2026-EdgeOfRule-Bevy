use bevy::prelude::*;

use crate::{
    assets::player::PlayerAssets,
    entities::player::{Player, spawn_player},
    levels::day1::scene1::DoorState,
};

pub fn spawn(commands: Commands, asset: Res<PlayerAssets>, door_state: Res<DoorState>) {
    if *door_state == DoorState::Closed {
        spawn_player(commands, asset, Transform::from_xyz(-234.0, 250.0, 0.0));
    } else {
        spawn_player(commands, asset, Transform::from_xyz(394.0, 100.0, 0.0));
    }
}

pub fn despawn(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player in &players {
        commands.entity(player).despawn();
    }
}
