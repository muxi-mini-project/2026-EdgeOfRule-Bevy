use bevy::prelude::*;

use crate::entities::ladder::{Ladder, spawn_ladder};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_ladder(
        &mut commands,
        Transform::from_xyz(-394.0, -78.0, -5.0),
        asset,
    );
}

pub fn despawn(mut commands: Commands, ladders: Query<Entity, With<Ladder>>) {
    for ladder in &ladders {
        commands.entity(ladder).despawn();
    }
}
