use bevy::prelude::*;

use crate::entities::sewage::{Sewage, spawn_sewage};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_sewage(&mut commands, Transform::from_xyz(0.0, -248.0, 1.0), &asset);
}

pub fn despawn(mut commands: Commands, keys: Query<Entity, With<Sewage>>) {
    for key in &keys {
        commands.entity(key).despawn_recursive();
    }
}
