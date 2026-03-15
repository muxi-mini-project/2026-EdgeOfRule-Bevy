use bevy::prelude::*;

use crate::entities::{hole::Hole, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct PressEtoEnterHole;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoEnterHole>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Hole>>,
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
                    Transform::from_xyz(-344.0, 300.0, 25.0),
                    &asset_server,
                    "返回",
                    PressEtoEnterHole,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoEnterHole>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Hole>>,
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

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoEnterHole>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
