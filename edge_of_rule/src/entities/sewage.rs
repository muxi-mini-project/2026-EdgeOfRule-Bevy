use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::SCALE;
use crate::physics::collision::CollisionGroup;

#[derive(Component)]
pub struct Sewage;

#[derive(Component)]
pub struct SewageSprite;

pub fn spawn_sewage(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
) {
    let texture = asset_server.load("images/animations/sewage.png");

    // Root stays unscaled so collider uses world sizes.
    commands
        .spawn((
            SpatialBundle {
                transform,
                ..default()
            },
            Sewage,
            Collider::cuboid((246.0 * SCALE) / 2.0, (22.0 * SCALE) / 2.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(CollisionGroup::Water.into(), CollisionGroup::Player.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(SCALE)),
                    ..default()
                },
                SewageSprite,
            ));
        });
}
