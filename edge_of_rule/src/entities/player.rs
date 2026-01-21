use crate::assets::player::PlayerAssets;
use crate::constants::{PLAYER_HEIGHT, PLAYER_WIDTH, SCALE};
use crate::entities::general::InGameEntity;
use crate::physics::collision::CollisionGroup;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FacingDirection {
    Left,
    #[default]
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Jumping,
    Falling,
    FastFalling,
    Crouching,
    Sliding,
    Dashing,
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    pub jump_count: usize,
    pub max_jumps: usize,
    pub is_grounded: bool,
    pub dash_speed: f32,
    pub dash_direction: Vec2,
    pub dash_timer: Timer,
    pub dash_cooldown_timer: Timer,
    pub state: PlayerState,
    pub facing: FacingDirection,
    pub ignore_down_input: bool,
}

pub fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: player_assets.front_texture.clone(),
            transform: Transform::from_xyz(-50.0, 20.0, 0.0).with_scale(Vec3::splat(SCALE)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0),
        Velocity::zero(),
        GravityScale(12.0),
        Ccd::enabled(),
        LockedAxes::ROTATION_LOCKED,
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        },
        Player {
            speed: 350.0,
            jump_force: 500.0,
            jump_count: 0,
            max_jumps: 2,
            is_grounded: false,
            dash_speed: 800.0,
            dash_direction: Vec2::ZERO,
            dash_timer: Timer::from_seconds(0.3, TimerMode::Once),
            dash_cooldown_timer: {
                let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
                timer.set_elapsed(timer.duration());
                timer
            },
            state: PlayerState::Idle,
            facing: FacingDirection::Right,
            ignore_down_input: false,
        },
        CollisionGroups::new(CollisionGroup::Player.into(), CollisionGroup::Ground.into()),
        ActiveEvents::COLLISION_EVENTS,
        InGameEntity,
    ));
}
