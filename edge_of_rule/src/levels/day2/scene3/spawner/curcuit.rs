use bevy::prelude::*;

use crate::entities::circuit::{spawn_curcuit, Circuit};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_curcuit(
        &mut commands,
        Transform::from_xyz(48.0, -24.0, -5.0),
        &asset,
    );
}

pub fn despawn(mut commands: Commands, ladders: Query<Entity, With<Circuit>>) {
    for ladder in &ladders {
        commands.entity(ladder).despawn();
    }
}
