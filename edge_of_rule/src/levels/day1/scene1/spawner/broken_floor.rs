use bevy::prelude::*;

use crate::entities::broken_floor::{BrokenFloor, spawn_broken_floor};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_broken_floor(
        &mut commands,
        Transform::from_xyz(208.0, -108.0, -5.0),
        &asset,
    );
    spawn_broken_floor(
        &mut commands,
        Transform::from_xyz(208.0, -144.0, -5.0),
        &asset,
    );
}

pub fn despawn(mut commands: Commands, floors: Query<Entity, With<BrokenFloor>>) {
    for floor in &floors {
        commands.entity(floor).despawn();
    }
}
