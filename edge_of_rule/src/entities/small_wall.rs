use bevy::prelude::*;

use crate::{constants::SCALE, entities::wall::spawn_wall};

#[derive(Component)]
pub struct SmallWall;

pub fn spawn_small_wall(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/small_wall.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        SmallWall,
    ));

    spawn_wall(
        commands,
        Vec2::new(12.0, 4.0),
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y + 48.0,
            transform.translation.z,
        ),
    );

    spawn_wall(
        commands,
        Vec2::new(12.0, 4.0),
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y - 48.0,
            transform.translation.z,
        ),
    );
    spawn_wall(
        commands,
        Vec2::new(4.0, 96.0),
        Transform::from_xyz(
            transform.translation.x - 4.0,
            transform.translation.y,
            transform.translation.z,
        ),
    );
    spawn_wall(
        commands,
        Vec2::new(4.0, 96.0),
        Transform::from_xyz(
            transform.translation.x + 4.0,
            transform.translation.y,
            transform.translation.z,
        ),
    );
}
