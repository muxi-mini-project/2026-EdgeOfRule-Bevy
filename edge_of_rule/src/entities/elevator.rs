use bevy::prelude::*;

use crate::{
    constants::SCALE,
    entities::{ground::spawn_ground, wall::spawn_wall},
};

#[derive(Component)]
pub struct Elevator;

pub fn spawn_elevator(
    commands: &mut Commands,
    transform: Transform,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/elevator/stop.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Elevator,
    ));
    spawn_ground(
        commands,
        Vec2::new(116.0, 4.0),
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y + 12.0,
            transform.translation.z,
        ),
    );

    spawn_wall(
        commands,
        Vec2::new(124.0, 4.0),
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y - 12.0,
            transform.translation.z,
        ),
    );
    spawn_wall(
        commands,
        Vec2::new(4.0, 28.0),
        Transform::from_xyz(
            transform.translation.x - 60.0,
            transform.translation.y,
            transform.translation.z,
        ),
    );
    spawn_wall(
        commands,
        Vec2::new(4.0, 28.0),
        Transform::from_xyz(
            transform.translation.x + 60.0,
            transform.translation.y,
            transform.translation.z,
        ),
    );
}
