use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, key::Key, player::Player},
    levels::day1::scene1::Picked,
};

#[derive(Component)]
pub struct ArrowOfKey;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfKey>,
    players: Query<&Transform, With<Player>>,
    keys: Query<&Transform, With<Key>>,
    picked: Res<Picked>,
) {
    if *picked != Picked::None {
        return;
    }
    if arrows.iter().len() != 0 {
        return;
    }

    for player in &players {
        for key in &keys {
            if (player.translation.x - key.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(key.translation.x, key.translation.y + 60.0, 1.0),
                    &asset_server,
                    ArrowOfKey,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfKey>>,
    players: Query<&Transform, With<Player>>,
    keys: Query<&Transform, With<Key>>,
) {
    for player in &players {
        for key in &keys {
            if (player.translation.x - key.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfKey>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
