use bevy::prelude::*;

use crate::entities::{mirror::Mirror, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct PressEtoLookMirror;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoLookMirror>,
    players: Query<&Transform, With<Player>>,
    mirrors: Query<&Transform, With<Mirror>>,
) {
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        for mirror in &mirrors {
            if (player.translation.x - mirror.translation.x).abs() < 60.0 {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(66.0, 92.0, 1.0),
                    &asset_server,
                    "查看",
                    PressEtoLookMirror,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoLookMirror>>,
    players: Query<&Transform, With<Player>>,
    mirrors: Query<&Transform, With<Mirror>>,
) {
    for player in &players {
        for mirror in &mirrors {
            if (player.translation.x - mirror.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoLookMirror>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
