use bevy::prelude::*;

use crate::{
    constants::SCALE,
    entities::{ground::spawn_ground, wall::spawn_wall},
};

#[derive(Component)]
pub struct SmallPlatform;

pub fn spawn_small_platform(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/small_platform.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        SmallPlatform,
    ));

    spawn_ground(
        commands,
        Vec2::new(128.0, 4.0),
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y + 4.0,
            transform.translation.z,
        ),
    );

    spawn_wall(
        commands,
        Vec2::new(128.0, 4.0),
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y - 4.0,
            transform.translation.z,
        ),
    );
    spawn_wall(
        commands,
        Vec2::new(4.0, 12.0),
        Transform::from_xyz(
            transform.translation.x - 64.0,
            transform.translation.y,
            transform.translation.z,
        ),
    );
    spawn_wall(
        commands,
        Vec2::new(4.0, 12.0),
        Transform::from_xyz(
            transform.translation.x + 64.0,
            transform.translation.y,
            transform.translation.z,
        ),
    );
}
