use bevy::prelude::*;

use crate::entities::door::{spawn_door, Door};

pub const LEFT_DOOR_POS: Vec3 = Vec3::new(-280.0, 16.0, -5.0);
pub const RIGHT_DOOR_POS: Vec3 = Vec3::new(280.0, 16.0, -5.0);

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_door(
        &mut commands,
        Transform::from_translation(LEFT_DOOR_POS),
        &asset,
    );
    spawn_door(
        &mut commands,
        Transform::from_translation(RIGHT_DOOR_POS),
        &asset,
    );
}

pub fn despawn(mut commands: Commands, doors: Query<Entity, With<Door>>) {
    for door in &doors {
        commands.entity(door).despawn();
    }
}
