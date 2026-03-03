use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, door::Door, player::Player, trapdoor::Trapdoor};

#[derive(Component)]
pub struct ArrowOfTrapdoor;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfTrapdoor>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Trapdoor>>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(116.0, -100.0, 1.0),
                    &asset_server,
                    ArrowOfTrapdoor,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfTrapdoor>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Trapdoor>>,
) {
    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfTrapdoor>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
