use bevy::prelude::*;

use crate::entities::small_platform::{spawn_small_platform, SmallPlatform};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-392.0, -186.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-264.0, -58.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-136.0, -186.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-392.0, 70.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-264.0, 70.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-8.0, -58.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-264.0, 190.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-136.0, 70.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-136.0, 190.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(-8.0, 190.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(120.0, -186.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(248.0, -186.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(376.0, -186.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(120.0, 70.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(376.0, 70.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(248.0, -58.0, -5.0),
        &asset,
    );
    spawn_small_platform(
        &mut commands,
        Transform::from_xyz(248.0, 190.0, -5.0),
        &asset,
    );
}

pub fn despawn(mut commands: Commands, ladders: Query<Entity, With<SmallPlatform>>) {
    for ladder in &ladders {
        commands.entity(ladder).despawn();
    }
}
