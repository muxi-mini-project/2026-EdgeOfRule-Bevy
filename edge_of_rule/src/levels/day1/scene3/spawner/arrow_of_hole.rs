use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, hole::Hole, player::Player};

#[derive(Component)]
pub struct ArrowOfHole;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfHole>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Hole>>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0
                && (player.translation.y - door.translation.y).abs() < 90.0
            {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(-394.0, 300.0, 25.0),
                    &asset_server,
                    ArrowOfHole,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfHole>>,
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

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfHole>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
