use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, chest::Chest, player::Player},
    levels::day1::scene3::Scene3ChestState,
};

#[derive(Component)]
pub struct ArrowOfChest;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfChest>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Chest>>,
    chest_state: Res<Scene3ChestState>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    if *chest_state == Scene3ChestState::Picked {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0
                && (player.translation.y - door.translation.y).abs() < 90.0
            {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(56.0, -80.0, 25.0),
                    &asset_server,
                    ArrowOfChest,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfChest>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Chest>>,
    chest_state: Res<Scene3ChestState>,
) {
    for player in &players {
        if *chest_state == Scene3ChestState::Picked {
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

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfChest>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
