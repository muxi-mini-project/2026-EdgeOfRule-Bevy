use bevy::prelude::*;

use crate::entities::{chest::Chest, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct PressEtoOpenChest;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoOpenChest>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Chest>>,
) {
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0
                && (player.translation.y - door.translation.y).abs() < 90.0
            {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(6.0, -80.0, 25.0),
                    &asset_server,
                    "打开",
                    PressEtoOpenChest,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoOpenChest>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Chest>>,
) {
    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0
                && (player.translation.y - door.translation.y).abs() < 90.0
            {
                return;
            }
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoOpenChest>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
