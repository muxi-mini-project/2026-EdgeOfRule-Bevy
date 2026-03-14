use bevy::prelude::*;

use crate::{
    entities::{door::Door, player::Player, press_e::spawn_press_e},
    levels::day1::scene1::Scene1DoorState,
};

#[derive(Component)]
pub struct PressEtoOpenDoor;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoOpenDoor>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Door>>,
    door_state: Res<Scene1DoorState>,
) {
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(480.0, 116.0, -4.0),
                    &asset_server,
                    if *door_state == Scene1DoorState::Closed {
                        "开门"
                    } else {
                        "进入"
                    },
                    PressEtoOpenDoor,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoOpenDoor>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Door>>,
    door_state: Res<Scene1DoorState>,
) {
    if door_state.is_changed() {
        for query in &querys {
            commands.entity(query).despawn_recursive();
        }
    }
    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoOpenDoor>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
