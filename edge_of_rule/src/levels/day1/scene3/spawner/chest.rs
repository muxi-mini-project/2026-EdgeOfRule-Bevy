use bevy::prelude::*;

use crate::entities::chest::{Chest, spawn_chest};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_chest(&mut commands, Transform::from_xyz(56.0, -136.0, 1.0), asset);
}

pub fn despawn(mut commands: Commands, keys: Query<Entity, With<Chest>>) {
    for key in &keys {
        commands.entity(key).despawn();
    }
}
