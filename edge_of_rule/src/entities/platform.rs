use bevy::prelude::*;

use crate::{
    constants::SCALE,
    entities::{ground::spawn_ground, wall::spawn_wall},
};

#[derive(Component)]
pub struct Platform;

pub fn spawn_platform(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/platform.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Platform,
    ));

    spawn_ground(
        commands,
        Vec2::new(172.0, 4.0),
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y + 24.0,
            transform.translation.z,
        ),
    );

    spawn_wall(
        commands,
        Vec2::new(180.0, 4.0),
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y - 8.0,
            transform.translation.z,
        ),
    );
    spawn_wall(
        commands,
        Vec2::new(4.0, 32.0),
        Transform::from_xyz(
            transform.translation.x - 88.0,
            transform.translation.y + 10.0,
            transform.translation.z,
        ),
    );
    spawn_wall(
        commands,
        Vec2::new(4.0, 32.0),
        Transform::from_xyz(
            transform.translation.x + 88.0,
            transform.translation.y + 10.0,
            transform.translation.z,
        ),
    );
}
