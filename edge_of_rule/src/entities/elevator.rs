use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::SCALE,
    entities::{ground::Ground, wall::Wall},
    physics::collision::CollisionGroup,
};

#[derive(Component)]
pub struct Elevator {
    pub min_y: f32,
    pub max_y: f32,
}

pub fn spawn_elevator(
    commands: &mut Commands,
    transform: Transform,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("images/animations/elevator/stop.png");

    // Root entity stays unscaled so colliders keep world sizes.
    commands
        .spawn((
            SpatialBundle {
                transform,
                ..default()
            },
            Elevator {
                // Default travel range; can be overridden later per-scene.
                min_y: transform.translation.y - 260.0,
                max_y: transform.translation.y,
            },
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture,
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(SCALE)),
                ..default()
            });

            spawn_elevator_ground(
                parent,
                Vec2::new(124.0, 4.0),
                Transform::from_xyz(0.0, 12.0, 0.0),
            );
            spawn_elevator_wall(
                parent,
                Vec2::new(124.0, 4.0),
                Transform::from_xyz(0.0, -12.0, 0.0),
            );
            spawn_elevator_wall(
                parent,
                Vec2::new(4.0, 20.0),
                Transform::from_xyz(-60.0, 0.0, 0.0),
            );
            spawn_elevator_wall(
                parent,
                Vec2::new(4.0, 20.0),
                Transform::from_xyz(60.0, 0.0, 0.0),
            );
        });
}

fn spawn_elevator_ground(parent: &mut ChildBuilder, custom_size: Vec2, transform: Transform) {
    parent.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(custom_size),
                ..default()
            },
            transform,
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(custom_size.x / 2.0, custom_size.y / 2.0),
        CollisionGroups::new(CollisionGroup::Ground.into(), Group::ALL),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        Ground,
    ));
}

fn spawn_elevator_wall(parent: &mut ChildBuilder, custom_size: Vec2, transform: Transform) {
    parent.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(custom_size),
                ..default()
            },
            transform,
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(custom_size.x / 2.0, custom_size.y / 2.0),
        CollisionGroups::new(CollisionGroup::Ground.into(), Group::ALL),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        Wall,
    ));
}
