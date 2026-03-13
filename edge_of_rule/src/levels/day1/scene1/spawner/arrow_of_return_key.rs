use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, key::Key, player::Player},
    levels::day1::scene1::Picked,
};

#[derive(Component)]
pub struct ArrowOfReturnKey;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfReturnKey>,
    players: Query<&Transform, With<Player>>,
    picked: Res<Picked>,
) {
    if *picked != Picked::Key {
        return;
    }
    if arrows.iter().len() != 0 {
        return;
    }

    for player in &players {
        if (player.translation.x - (-220.0)).abs() < 60.0 {
            spawn_arrow(
                &mut commands,
                Transform::from_xyz(-220.0, -64.0 + 60.0, 1.0),
                &asset_server,
                ArrowOfReturnKey,
            );
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfReturnKey>>,
    players: Query<&Transform, With<Player>>,
    keys: Query<&Key>,
) {
    for player in &players {
        if (player.translation.x - (-220.0)).abs() < 60.0 && keys.iter().len() == 0 {
            return;
        }
    }

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfReturnKey>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
