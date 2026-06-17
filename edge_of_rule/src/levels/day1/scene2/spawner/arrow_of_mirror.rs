use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, mirror::Mirror, player::Player};

#[derive(Component)]
pub struct ArrowOfMirror;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfMirror>,
    players: Query<&Transform, With<Player>>,
    mirrors: Query<&Transform, With<Mirror>>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    for player in &players {
        for mirror in &mirrors {
            if (player.translation.x - mirror.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(-6.0, 92.0, 1.0),
                    &asset_server,
                    ArrowOfMirror,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfMirror>>,
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

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfMirror>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
