use bevy::prelude::*;

use crate::entities::hole::{spawn_hole, Hole};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_hole(
        &mut commands,
        Transform::from_xyz(-394.0, 252.0, -5.0),
        asset,
    );
}

pub fn despawn(mut commands: Commands, ladders: Query<Entity, With<Hole>>) {
    for ladder in &ladders {
        commands.entity(ladder).despawn();
    }
}
