use bevy::prelude::*;

use crate::entities::trapdoor::{Trapdoor, spawn_trapdoor};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_trapdoor(
        &mut commands,
        Transform::from_xyz(116.0, -140.0, -5.0),
        asset,
    );
}

pub fn despawn(mut commands: Commands, doors: Query<Entity, With<Trapdoor>>) {
    for door in &doors {
        commands.entity(door).despawn();
    }
}
