use bevy::prelude::*;

use crate::{
    entities::screw::{spawn_screw, Screw},
    levels::day1::scene1::Picked,
};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>, picked: Res<Picked>) {
    if *picked == Picked::Screw {
        return;
    }
    spawn_screw(
        &mut commands,
        Transform::from_xyz(-396.0, -64.0, -5.0),
        asset,
    );
}

pub fn despawn(mut commands: Commands, screws: Query<Entity, With<Screw>>) {
    for screw in &screws {
        commands.entity(screw).despawn();
    }
}
