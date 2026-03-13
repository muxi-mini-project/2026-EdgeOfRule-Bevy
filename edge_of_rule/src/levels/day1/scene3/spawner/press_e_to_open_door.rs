use bevy::prelude::*;

use crate::{
    entities::{door::Door, player::Player, press_e::spawn_press_e},
    levels::day1::scene1::DoorState,
};

#[derive(Component)]
pub struct PressEtoOpenDoor;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoOpenDoor>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Door>>,
    door_state: Res<DoorState>,
) {
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0
                && (player.translation.y - door.translation.y).abs() < 90.0
            {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(468.0, 268.0, 25.0),
                    &asset_server,
                    if *door_state == DoorState::Closed {
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
    door_state: Res<DoorState>,
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
