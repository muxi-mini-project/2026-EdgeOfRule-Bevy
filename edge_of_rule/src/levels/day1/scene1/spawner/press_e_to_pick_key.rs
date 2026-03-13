use bevy::prelude::*;

use crate::{
    entities::{key::Key, player::Player, press_e::spawn_press_e},
    levels::day1::scene1::Picked,
};

#[derive(Component)]
pub struct PressEtoPickKey;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoPickKey>,
    players: Query<&Transform, With<Player>>,
    keys: Query<&Transform, With<Key>>,
    picked: Res<Picked>,
) {
    if *picked != Picked::None {
        return;
    }
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        for key in &keys {
            if (player.translation.x - key.translation.x).abs() < 60.0 {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(key.translation.x + 72.0, key.translation.y + 60.0, 1.0),
                    &asset_server,
                    "拾取钥匙",
                    PressEtoPickKey,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoPickKey>>,
    players: Query<&Transform, With<Player>>,
    keys: Query<&Transform, With<Key>>,
) {
    for player in &players {
        for key in &keys {
            if (player.translation.x - key.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoPickKey>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
