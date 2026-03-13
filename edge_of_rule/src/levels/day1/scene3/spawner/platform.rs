use bevy::prelude::*;

use crate::entities::platform::{Platform, spawn_platform};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        Transform::from_xyz(-234.0, 70.0, -5.0),
        &asset,
    );
    spawn_platform(
        &mut commands,
        Transform::from_xyz(-74.0, -50.0, -5.0),
        &asset,
    );
    spawn_platform(
        &mut commands,
        Transform::from_xyz(86.0, -170.0, -5.0),
        &asset,
    );
    spawn_platform(
        &mut commands,
        Transform::from_xyz(394.0, 70.0, -5.0),
        &asset,
    );
}

pub fn despawn(mut commands: Commands, platforms: Query<Entity, With<Platform>>) {
    for platform in &platforms {
        commands.entity(platform).despawn();
    }
}
