use bevy::prelude::*;

use crate::entities::windows::{Windows, spawn_windows};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_windows(
        &mut commands,
        Transform::from_xyz(-40.0, 16.0, -5.0),
        &asset,
    );
    spawn_windows(&mut commands, Transform::from_xyz(68.0, 16.0, -5.0), &asset);
    spawn_windows(
        &mut commands,
        Transform::from_xyz(176.0, 16.0, -5.0),
        &asset,
    );
}

pub fn despawn(mut commands: Commands, doors: Query<Entity, With<Windows>>) {
    for door in &doors {
        commands.entity(door).despawn();
    }
}
