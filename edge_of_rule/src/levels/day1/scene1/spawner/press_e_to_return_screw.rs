use bevy::prelude::*;

use crate::{
    entities::{player::Player, press_e::spawn_press_e, screw::Screw},
    levels::day1::scene1::Picked,
};

#[derive(Component)]
pub struct PressEtoReturnScrew;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoReturnScrew>,
    players: Query<&Transform, With<Player>>,
    picked: Res<Picked>,
) {
    if *picked != Picked::Screw {
        return;
    }
    if querys.iter().len() != 0 {
        return;
    }

    for player in &players {
        if (player.translation.x - (-396.0)).abs() < 60.0 {
            spawn_press_e(
                &mut commands,
                Transform::from_xyz(-396.0 + 72.0, -64.0 + 60.0, 1.0),
                &asset_server,
                "放下改锥",
                PressEtoReturnScrew,
            );
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoReturnScrew>>,
    players: Query<&Transform, With<Player>>,
    screws: Query<&Screw>,
) {
    for player in &players {
        if (player.translation.x - (-396.0)).abs() < 60.0 && screws.iter().len() == 0 {
            return;
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoReturnScrew>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
