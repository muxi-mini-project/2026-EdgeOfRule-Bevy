use bevy::prelude::*;

use crate::entities::log::{Log, spawn_log};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_log(&mut commands, Transform::from_xyz(24.0, -18.0, -4.0), asset);
}

pub fn despawn(mut commands: Commands, doors: Query<Entity, With<Log>>) {
    for door in &doors {
        commands.entity(door).despawn();
    }
}
