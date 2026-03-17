use bevy::prelude::*;

use crate::{
    entities::{player::Player, press_e::spawn_press_e},
    levels::day1::scene1::Day1Finished,
};

#[derive(Component)]
pub struct PressEtoSleep;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoSleep>,
    players: Query<&Transform, With<Player>>,
    finished: Res<Day1Finished>,
) {
    if querys.iter().len() != 0 {
        return;
    }

    if *finished != Day1Finished::Yes {
        return;
    }

    for player in &players {
        if (player.translation.x - (-120.0)).abs() < 60.0 {
            spawn_press_e(
                &mut commands,
                Transform::from_xyz(-70.0, -4.0, 1.0),
                &asset_server,
                "睡觉",
                PressEtoSleep,
            );
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoSleep>>,
    players: Query<&Transform, With<Player>>,
) {
    for player in &players {
        if (player.translation.x - (-120.0)).abs() < 60.0 {
            return;
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoSleep>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
