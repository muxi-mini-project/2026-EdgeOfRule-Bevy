use bevy::prelude::*;

use crate::entities::hole::{Hole, spawn_hole};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_hole(
        &mut commands,
        Transform::from_xyz(104.0, -24.0, -5.0),
        asset,
    );
}

pub fn despawn(mut commands: Commands, ladders: Query<Entity, With<Hole>>) {
    for ladder in &ladders {
        commands.entity(ladder).despawn();
    }
}
