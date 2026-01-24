use bevy::prelude::*;

use crate::entities::{door::Door, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct PressEtoBackScene1;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoBackScene1>,
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
                    Transform::from_xyz(-24.0, 116.0, -4.0),
                    &asset_server,
                    "返回",
                    PressEtoBackScene1,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoBackScene1>>,
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

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoBackScene1>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
