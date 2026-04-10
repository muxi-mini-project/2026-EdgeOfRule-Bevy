use bevy::prelude::*;

use crate::entities::windows::{spawn_windows, Windows};

pub const WINDOWS_POSITIONS: [f32; 3] = [-40.0, 68.0, 176.0];

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    for x in WINDOWS_POSITIONS {
        spawn_windows(&mut commands, Transform::from_xyz(x, 16.0, -5.0), &asset);
    }
}

pub fn despawn(mut commands: Commands, doors: Query<Entity, With<Windows>>) {
    for door in &doors {
        commands.entity(door).despawn();
    }
}
