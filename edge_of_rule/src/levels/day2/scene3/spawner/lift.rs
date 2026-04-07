use bevy::prelude::*;

use crate::assets::lift::LiftAssets;
use crate::entities::lift::{spawn_lift, Lift};

pub fn spawn(mut commands: Commands, assets: Res<LiftAssets>) {
    spawn_lift(
        &mut commands,
        Transform::from_xyz(-48.0, 26.0, -5.0),
        &assets,
    );
}

pub fn despawn(mut commands: Commands, ladders: Query<Entity, With<Lift>>) {
    for ladder in &ladders {
        commands.entity(ladder).despawn();
    }
}
