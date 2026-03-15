use bevy::prelude::*;

use crate::entities::pipe::{spawn_pipe, Pipe};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_pipe(&mut commands, Transform::from_xyz(-184.0, 0.0, -5.0), asset);
}

pub fn despawn(mut commands: Commands, pipes: Query<Entity, With<Pipe>>) {
    for pipe in &pipes {
        commands.entity(pipe).despawn();
    }
}
