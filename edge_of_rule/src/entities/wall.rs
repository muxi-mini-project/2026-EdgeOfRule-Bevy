use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::physics::collision::CollisionGroup;

#[derive(Component)]
pub struct Wall;

pub fn spawn_wall(commands: &mut Commands, custom_size: Vec2, transform: Transform) {
    commands.spawn((
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
