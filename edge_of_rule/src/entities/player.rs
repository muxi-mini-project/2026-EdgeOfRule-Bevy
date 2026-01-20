use crate::assets::player::PlayerAssets;
use crate::constants::{PLAYER_HEIGHT, PLAYER_WIDTH};
use crate::entities::general::InGameEntity;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
    jump_force: f32,
    jump_count: usize,
    max_jumps: usize,
    is_grounded: bool,
    spawn_point: Vec2,
}

pub fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: player_assets.texture.clone(),
            transform: Transform::from_xyz(50.0, -50.0, 0.0).with_scale(Vec3::new(4.0, 4.0, 0.0)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(PLAYER_WIDTH * 2.0, PLAYER_HEIGHT * 2.0),
        Velocity::zero(),
        GravityScale(5.0),
        Ccd::enabled(),
        LockedAxes::ROTATION_LOCKED,
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Damping {
            linear_damping: 0.5,
            angular_damping: 0.0,
        },
        Player {
            speed: 300.0,
            jump_force: 400.0,
            jump_count: 0,
            max_jumps: 2,
            is_grounded: false,
            spawn_point: Vec2::new(50.0, -50.0),
        },
        ActiveEvents::COLLISION_EVENTS,
        InGameEntity,
    ));
}
