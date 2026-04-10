use bevy::prelude::*;

use crate::entities::{
    ground::{spawn_ground, Ground},
    wall::{spawn_wall, Wall},
};

pub fn spawn(mut commands: Commands) {
    spawn_ground(
        &mut commands,
        Vec2::new(1000.0, 50.0),
        Transform::from_xyz(0.0, -150.0, 0.0),
    );
    spawn_wall(
        &mut commands,
        Vec2::new(1000.0, 50.0),
        Transform::from_xyz(0.0, 225.0, 0.0),
    );

    spawn_wall(
        &mut commands,
        Vec2::new(50.0, 500.0),
        Transform::from_xyz(-189.0, 0.0, 0.0),
    );
    spawn_wall(
        &mut commands,
        Vec2::new(50.0, 500.0),
        Transform::from_xyz(189.0, 0.0, 0.0),
    );
}

pub fn despawn(
    mut commands: Commands,
    grounds: Query<Entity, With<Ground>>,
    walls: Query<Entity, With<Wall>>,
) {
    for ground in &grounds {
        commands.entity(ground).despawn();
    }
    for wall in &walls {
        commands.entity(wall).despawn();
    }
}
