use bevy::prelude::*;

use crate::entities::small_note::{SmallNote, spawn_small_note};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_small_note(&mut commands, Transform::from_xyz(8.0, -58.0, -4.0), asset);
}

pub fn despawn(mut commands: Commands, doors: Query<Entity, With<SmallNote>>) {
    for door in &doors {
        commands.entity(door).despawn();
    }
}
