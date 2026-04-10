use bevy::prelude::*;

use crate::entities::desk::{spawn_desk, Desk};

pub const DESK_Y: f32 = -68.0;
pub const DESK_Z: f32 = -5.0;

pub const DESK_POSITIONS: [Vec3; 3] = [
    Vec3::new(-160.0, DESK_Y, DESK_Z),
    Vec3::new(0.0, DESK_Y, DESK_Z),
    Vec3::new(160.0, DESK_Y, DESK_Z),
];

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    for (i, pos) in DESK_POSITIONS.iter().copied().enumerate() {
        spawn_desk(&mut commands, Transform::from_translation(pos), &asset, i);
    }
}

pub fn despawn(mut commands: Commands, desks: Query<Entity, With<Desk>>) {
    for desk in &desks {
        commands.entity(desk).despawn();
    }
}
