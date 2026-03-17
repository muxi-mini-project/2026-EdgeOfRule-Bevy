use bevy::prelude::*;

use crate::{
    entities::{player::Player, press_e::spawn_press_e, trapdoor::Trapdoor},
    levels::day1::{scene1::Picked, scene2::TrapdoorState},
};

#[derive(Component)]
pub struct PressEtoEnterTrapdoor;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoEnterTrapdoor>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Trapdoor>>,
    picked: Res<Picked>,
    trapdoor_state: Res<TrapdoorState>,
) {
    if querys.iter().len() != 0 {
        return;
    }

    if *picked != Picked::Screw && *trapdoor_state == TrapdoorState::Closed {
        return;
    }
    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(188.0, -100.0, 1.0),
                    &asset_server,
                    "进入",
                    PressEtoEnterTrapdoor,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoEnterTrapdoor>>,
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

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoEnterTrapdoor>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
