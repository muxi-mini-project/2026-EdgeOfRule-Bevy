use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, log::Log, player::Player};

#[derive(Component)]
pub struct ArrowOfLog;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfLog>,
    players: Query<&Transform, With<Player>>,
    notes: Query<&Transform, With<Log>>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    for player in &players {
        for note in &notes {
            if (player.translation.x - note.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(24.0, 2.0, 25.0),
                    &asset_server,
                    ArrowOfLog,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfLog>>,
    players: Query<&Transform, With<Player>>,
    notes: Query<&Transform, With<Log>>,
) {
    for player in &players {
        for note in &notes {
            if (player.translation.x - note.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfLog>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
