use bevy::prelude::*;

use crate::entities::door::{spawn_door, Door};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_door(&mut commands, Transform::from_xyz(-96.0, 16.0, -5.0), asset);
}

pub fn despawn(mut commands: Commands, doors: Query<Entity, With<Door>>) {
    for door in &doors {
        commands.entity(door).despawn();
    }
}
