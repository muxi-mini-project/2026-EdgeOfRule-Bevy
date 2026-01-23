use bevy::prelude::*;

use crate::entities::mirror::{Mirror, spawn_mirror};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_mirror(&mut commands, Transform::from_xyz(-6.0, 32.0, -5.0), asset);
}

pub fn despawn(mut commands: Commands, mirrors: Query<Entity, With<Mirror>>) {
    for mirror in &mirrors {
        commands.entity(mirror).despawn();
    }
}
