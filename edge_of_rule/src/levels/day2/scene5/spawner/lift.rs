use bevy::prelude::*;

use crate::{
    assets::lift::LiftAssets,
    entities::lift::{spawn_lift, Lift},
};

pub fn spawn(mut commands: Commands, assets: Res<LiftAssets>) {
    spawn_lift(
        &mut commands,
        Transform::from_xyz(-268.0, 26.0, -5.0),
        &assets,
    );
}

pub fn despawn(mut commands: Commands, lifts: Query<Entity, With<Lift>>) {
    for lift in &lifts {
        commands.entity(lift).despawn();
    }
}
