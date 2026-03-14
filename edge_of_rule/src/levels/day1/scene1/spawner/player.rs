use bevy::prelude::*;

use crate::{
    assets::player::PlayerAssets,
    entities::player::{Player, spawn_player},
    levels::day1::scene1::Scene1DoorState,
};

pub fn spawn(commands: Commands, asset: Res<PlayerAssets>, door_state: Res<Scene1DoorState>) {
    if *door_state == Scene1DoorState::Opened {
        spawn_player(commands, asset, Transform::from_xyz(408.0, -68.0, 0.0))
    } else {
        spawn_player(commands, asset, Transform::from_xyz(-100.0, -68.0, 0.0))
    }
}

pub fn despawn(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player in &players {
        commands.entity(player).despawn();
    }
}
