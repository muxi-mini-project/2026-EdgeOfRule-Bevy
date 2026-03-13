use bevy::prelude::*;

use crate::entities::{player::Player, press_e::spawn_press_e, small_note::SmallNote};

#[derive(Component)]
pub struct PressEtoRead;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoRead>,
    players: Query<&Transform, With<Player>>,
    notes: Query<&Transform, With<SmallNote>>,
) {
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        for note in &notes {
            if (player.translation.x - note.translation.x).abs() < 60.0 {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(80.0, 2.0, 1.0),
                    &asset_server,
                    "阅读",
                    PressEtoRead,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoRead>>,
    players: Query<&Transform, With<Player>>,
    notes: Query<&Transform, With<SmallNote>>,
) {
    for player in &players {
        for note in &notes {
            if (player.translation.x - note.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoRead>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
