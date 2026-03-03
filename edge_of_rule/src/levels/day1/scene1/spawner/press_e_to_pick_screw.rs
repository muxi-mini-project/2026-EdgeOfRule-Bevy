use bevy::prelude::*;

use crate::entities::{player::Player, press_e::spawn_press_e, screw::Screw};

#[derive(Component)]
pub struct PressEtoPickScrew;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoPickScrew>,
    players: Query<&Transform, With<Player>>,
    screws: Query<&Transform, With<Screw>>,
) {
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        for screw in &screws {
            if (player.translation.x - screw.translation.x).abs() < 60.0 {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(
                        screw.translation.x + 72.0,
                        screw.translation.y + 60.0,
                        1.0,
                    ),
                    &asset_server,
                    "拾取改锥",
                    PressEtoPickScrew,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoPickScrew>>,
    players: Query<&Transform, With<Player>>,
    screws: Query<&Transform, With<Screw>>,
) {
    for player in &players {
        for screw in &screws {
            if (player.translation.x - screw.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoPickScrew>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
