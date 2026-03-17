use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, player::Player},
    levels::day1::scene1::Day1Finished,
};

#[derive(Component)]
pub struct ArrowOfSleep;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfSleep>,
    players: Query<&Transform, With<Player>>,
    finished: Res<Day1Finished>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    if *finished != Day1Finished::Yes {
        return;
    }

    for player in &players {
        if (player.translation.x - (-120.0)).abs() < 60.0 {
            spawn_arrow(
                &mut commands,
                Transform::from_xyz(-120.0, -64.0 + 60.0, 1.0),
                &asset_server,
                ArrowOfSleep,
            );
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfSleep>>,
    players: Query<&Transform, With<Player>>,
) {
    for player in &players {
        if (player.translation.x - (-120.0)).abs() < 60.0 {
            return;
        }
    }

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfSleep>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
