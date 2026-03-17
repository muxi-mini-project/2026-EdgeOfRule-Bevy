use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, cover::Cover, player::Player},
    levels::day1::scene3::Scene3CoverState,
};

#[derive(Component)]
pub struct ArrowOfCover;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfCover>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Cover>>,
    chest_state: Res<Scene3CoverState>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    if *chest_state == Scene3CoverState::Picked {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0
                && (player.translation.y - door.translation.y).abs() < 90.0
            {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(-304.0, -38.0, 25.0),
                    &asset_server,
                    ArrowOfCover,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfCover>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Cover>>,
    chest_state: Res<Scene3CoverState>,
) {
    for player in &players {
        if *chest_state == Scene3CoverState::Picked {
            break;
        }
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

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfCover>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
