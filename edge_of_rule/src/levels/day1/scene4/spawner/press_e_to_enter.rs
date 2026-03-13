use bevy::prelude::*;

use crate::entities::{door::Door, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct PressEtoEnter;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoEnter>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Door>>,
) {
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(-24.0, 116.0, 25.0),
                    &asset_server,
                    "返回",
                    PressEtoEnter,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoEnter>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Door>>,
) {
    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoEnter>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
