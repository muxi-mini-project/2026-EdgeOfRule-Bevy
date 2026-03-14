use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, door::Door, player::Player},
    levels::day1::scene1::{Picked, Scene1DoorState},
};

#[derive(Component)]
pub struct ArrowOfDoor;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfDoor>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Door>>,
    picked: Res<Picked>,
    door_state: Res<Scene1DoorState>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    if *picked != Picked::Key && *door_state == Scene1DoorState::Closed {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(408.0, 116.0, -4.0),
                    &asset_server,
                    ArrowOfDoor,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfDoor>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Door>>,
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

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfDoor>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
