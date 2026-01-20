use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::physics::collision::CollisionGroup;

#[derive(Component)]
pub struct Ground;

pub fn spawn_ground(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(1000.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -150.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(500.0, 25.0),
        CollisionGroups::new(CollisionGroup::Ground.into(), Group::ALL),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        Ground,
    ));
}
