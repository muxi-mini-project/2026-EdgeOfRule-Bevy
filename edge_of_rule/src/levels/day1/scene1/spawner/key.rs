use bevy::prelude::*;

use crate::entities::key::{Key, spawn_key};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_key(
        &mut commands,
        Transform::from_xyz(-220.0, -64.0, -5.0),
        asset,
    );
}

pub fn despawn(mut commands: Commands, keys: Query<Entity, With<Key>>) {
    for key in &keys {
        commands.entity(key).despawn();
    }
}
