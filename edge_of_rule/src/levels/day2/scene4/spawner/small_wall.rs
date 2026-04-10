use bevy::prelude::*;

use crate::entities::small_wall::{spawn_small_wall, SmallWall};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_small_wall(
        &mut commands,
        Transform::from_xyz(-66.0, -130.0, -5.0),
        &asset,
    );
    spawn_small_wall(
        &mut commands,
        Transform::from_xyz(-66.0, -242.0, -5.0),
        &asset,
    );
    spawn_small_wall(
        &mut commands,
        Transform::from_xyz(-194.0, -2.0, -5.0),
        &asset,
    );
    spawn_small_wall(
        &mut commands,
        Transform::from_xyz(62.0, 244.0, -5.0),
        &asset,
    );
    spawn_small_wall(
        &mut commands,
        Transform::from_xyz(190.0, 126.0, -5.0),
        &asset,
    );
    spawn_small_wall(
        &mut commands,
        Transform::from_xyz(190.0, -2.0, -5.0),
        &asset,
    );
    spawn_small_wall(
        &mut commands,
        Transform::from_xyz(318.0, 244.0, -5.0),
        &asset,
    );
}

pub fn despawn(mut commands: Commands, ladders: Query<Entity, With<SmallWall>>) {
    for ladder in &ladders {
        commands.entity(ladder).despawn();
    }
}
