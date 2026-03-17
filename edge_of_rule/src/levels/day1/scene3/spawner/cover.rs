use bevy::prelude::*;

use crate::entities::cover::{Cover, spawn_cover};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_cover(
        &mut commands,
        Transform::from_xyz(-304.0, -94.0, -4.0),
        asset,
    );
}

pub fn despawn(mut commands: Commands, keys: Query<Entity, With<Cover>>) {
    for key in &keys {
        commands.entity(key).despawn();
    }
}
