use bevy::prelude::*;

use crate::entities::elevator::{Elevator, spawn_elevator};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_elevator(
        &mut commands,
        Transform::from_xyz(-398.0, 82.0, -4.0),
        asset,
    );
}

pub fn despawn(mut commands: Commands, ladders: Query<Entity, With<Elevator>>) {
    for ladder in &ladders {
        commands.entity(ladder).despawn();
    }
}
