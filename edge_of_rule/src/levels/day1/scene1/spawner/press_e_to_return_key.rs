use bevy::prelude::*;

use crate::{
    entities::{key::Key, player::Player, press_e::spawn_press_e},
    levels::day1::scene1::Picked,
};

#[derive(Component)]
pub struct PressEtoReturnKey;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoReturnKey>,
    players: Query<&Transform, With<Player>>,
    picked: Res<Picked>,
) {
    if *picked != Picked::Key {
        return;
    }
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        if (player.translation.x - (-220.0)).abs() < 60.0 {
            spawn_press_e(
                &mut commands,
                Transform::from_xyz(-220.0 + 72.0, -64.0 + 60.0, 1.0),
                &asset_server,
                "放下钥匙",
                PressEtoReturnKey,
            );
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoReturnKey>>,
    players: Query<&Transform, With<Player>>,
    keys: Query<&Key>,
) {
    for player in &players {
        if (player.translation.x - (-220.0)).abs() < 60.0 && keys.iter().len() == 0 {
            return;
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoReturnKey>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
