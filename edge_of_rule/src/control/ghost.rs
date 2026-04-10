use bevy::prelude::*;

use crate::{
    animation::hurt_shake::HurtShake,
    core::{health::PlayerDied, health::PlayerHealth, state::GameState},
    entities::{
        ghost::{spawn_ghost, Ghost, GhostAssets, GhostMotion},
        player::Player,
    },
};

#[derive(Component)]
pub struct GhostCountdownRoot;

#[derive(Component)]
pub struct GhostCountdownText;

const COUNTDOWN_Z: i32 = 999;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GhostMode {
    Inactive,
    Active,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct Day2GhostManager {
    pub mode: GhostMode,
    pub dwell_seconds: f32,
    pub last_state: Option<GameState>,
}

impl Default for Day2GhostManager {
    fn default() -> Self {
        Self {
            mode: GhostMode::Inactive,
            dwell_seconds: 0.0,
            last_state: None,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct GhostFly {
    pub speed: f32,
    pub a: f32,
    pub b: f32,
    pub omega: f32,
    pub t: f32,
    pub phase: f32,
    pub hit_cooldown: f32,
}

fn is_day2(state: GameState) -> bool {
    matches!(
        state,
        GameState::Day2Scene1
            | GameState::Day2Scene2
            | GameState::Day2Scene3
            | GameState::Day2Scene4
            | GameState::Day2Scene5
    )
}

pub fn ghost_update_mode_system(
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut manager: ResMut<Day2GhostManager>,
) {
    let s = *state.get();

    if !is_day2(s) {
        manager.mode = GhostMode::Inactive;
        manager.dwell_seconds = 0.0;
        return;
    }

    if s == GameState::Day2Scene2 {
        manager.mode = GhostMode::Inactive;
        manager.dwell_seconds = 0.0;
        return;
    }

    if manager.mode == GhostMode::Inactive {
        manager.dwell_seconds += time.delta_seconds();
        if manager.dwell_seconds >= 10.0 {
            manager.mode = GhostMode::Active;
        }
    }
}

pub fn ghost_countdown_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                z_index: ZIndex::Global(COUNTDOWN_Z),
                visibility: Visibility::Hidden,
                ..default()
            },
            GhostCountdownRoot,
        ))
        .with_children(|root| {
            root.spawn((
                TextBundle {
                    text: Text::from_section(
                        "00:00.000",
                        TextStyle {
                            font: asset_server.load("font/font/aLiFont.ttf"),
                            font_size: 28.0,
                            color: Color::rgb(1.0, 0.0, 0.0),
                        },
                    ),
                    ..default()
                },
                GhostCountdownText,
            ));
        });
}

fn should_show_countdown(state: GameState) -> bool {
    matches!(
        state,
        GameState::Day2Scene1
            | GameState::Day2Scene3
            | GameState::Day2Scene4
            | GameState::Day2Scene5
    )
}

pub fn ghost_countdown_update_system(
    state: Res<State<GameState>>,
    manager: Res<Day2GhostManager>,
    mut root_vis: Query<&mut Visibility, With<GhostCountdownRoot>>,
    mut text: Query<&mut Text, With<GhostCountdownText>>,
) {
    let s = *state.get();
    let show = should_show_countdown(s);

    for mut v in &mut root_vis {
        *v = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if !show {
        return;
    }

    // Ghost activation countdown: 10s in Day2 (excluding Scene2).
    let remaining = if manager.mode == GhostMode::Inactive {
        (10.0 - manager.dwell_seconds).max(0.0)
    } else {
        0.0
    };

    let total_ms = (remaining * 1000.0).round().max(0.0) as i64;
    let minutes = (total_ms / 60000) as i64;
    let seconds = ((total_ms % 60000) / 1000) as i64;
    let millis = (total_ms % 1000) as i64;
    let value = format!("{:02}:{:02}.{:03}", minutes, seconds, millis);

    for mut t in &mut text {
        if let Some(sec) = t.sections.get_mut(0) {
            sec.value = value.clone();
        }
    }
}

pub fn ghost_scene_change_cleanup_system(
    mut commands: Commands,
    state: Res<State<GameState>>,
    mut manager: ResMut<Day2GhostManager>,
    ghosts: Query<Entity, With<Ghost>>,
) {
    let s = *state.get();
    if manager.last_state == Some(s) {
        return;
    }
    manager.last_state = Some(s);

    for e in &ghosts {
        commands.entity(e).despawn_recursive();
    }
}

pub fn ghost_despawn_system(
    mut commands: Commands,
    state: Res<State<GameState>>,
    manager: Res<Day2GhostManager>,
    ghosts: Query<Entity, With<Ghost>>,
) {
    let s = *state.get();
    let should_exist =
        is_day2(s) && s != GameState::Day2Scene2 && manager.mode == GhostMode::Active;

    if should_exist {
        return;
    }

    for e in &ghosts {
        commands.entity(e).despawn_recursive();
    }
}

pub fn ghost_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    manager: Res<Day2GhostManager>,
    players: Query<&Transform, With<Player>>,
    ghosts: Query<Entity, With<Ghost>>,
) {
    let s = *state.get();
    if !(is_day2(s) && s != GameState::Day2Scene2 && manager.mode == GhostMode::Active) {
        return;
    }
    if ghosts.iter().next().is_some() {
        return;
    }

    let Ok(player_tf) = players.get_single() else {
        return;
    };

    let mut pos = player_tf.translation + Vec3::new(180.0, 140.0, 0.0);
    pos.z = 10.0;
    spawn_ghost(
        &mut commands,
        Transform::from_translation(pos),
        &asset_server,
    );
}

pub fn ghost_attach_fly_params_system(
    mut commands: Commands,
    ghosts: Query<Entity, (With<Ghost>, Without<GhostFly>)>,
) {
    for e in &ghosts {
        commands.entity(e).insert(GhostFly {
            speed: 320.0,
            a: 620.0,
            b: 360.0,
            omega: 1.2,
            t: 0.0,
            phase: 0.35,
            hit_cooldown: 0.0,
        });
    }
}

pub fn ghost_movement_system(
    time: Res<Time>,
    players: Query<&Transform, (With<Player>, Without<Ghost>)>,
    mut ghosts: Query<
        (&mut Transform, &mut GhostFly, &mut GhostMotion),
        (With<Ghost>, Without<Player>),
    >,
) {
    let Ok(player_tf) = players.get_single() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    let dt = time.delta_seconds();
    for (mut tf, mut fly, mut motion) in &mut ghosts {
        let pos2 = tf.translation.truncate();

        fly.t += fly.omega * dt;
        if fly.t > std::f32::consts::TAU {
            fly.t -= std::f32::consts::TAU;
        }

        // Figure-eight (Lissajous) orbit around the player.
        // x = a * sin(t + phase)
        // y = b * sin(2t)
        let desired = player_pos
            + Vec2::new(
                fly.a * (fly.t + fly.phase).sin(),
                fly.b * (2.0 * fly.t).sin(),
            );

        let to = desired - pos2;
        let dist = to.length();
        let max_step = fly.speed * dt;
        let step = if dist <= max_step || dist < 1e-3 {
            to
        } else {
            to / dist * max_step
        };

        motion.last_pos = pos2;
        tf.translation.x += step.x;
        tf.translation.y += step.y;
    }
}

fn segment_intersects_circle(a: Vec2, b: Vec2, center: Vec2, radius: f32) -> bool {
    let ab = b - a;
    let ac = center - a;
    let ab_len2 = ab.length_squared();
    if ab_len2 <= 1e-6 {
        return (a - center).length_squared() <= radius * radius;
    }
    let t = (ac.dot(ab) / ab_len2).clamp(0.0, 1.0);
    let closest = a + ab * t;
    (closest - center).length_squared() <= radius * radius
}

pub fn ghost_damage_system(
    mut commands: Commands,
    time: Res<Time>,
    mut health: ResMut<PlayerHealth>,
    mut died_writer: EventWriter<PlayerDied>,
    players: Query<(Entity, &Transform), (With<Player>, Without<Ghost>)>,
    mut ghosts: Query<(&Transform, &mut GhostFly, &GhostMotion), (With<Ghost>, Without<Player>)>,
) {
    if health.dead {
        return;
    }
    let Ok((player_entity, player_tf)) = players.get_single() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    let dt = time.delta_seconds();
    // Easier to trigger than strict "pass-through": wider segment radius plus a near-player check.
    const HIT_RADIUS: f32 = 64.0;
    const HIT_RADIUS_NEAR: f32 = 52.0;
    for (tf, mut fly, motion) in &mut ghosts {
        fly.hit_cooldown = (fly.hit_cooldown - dt).max(0.0);
        if fly.hit_cooldown > 0.0 {
            continue;
        }

        let a = motion.last_pos;
        let b = tf.translation.truncate();
        let near_now = (b - player_pos).length_squared() <= HIT_RADIUS_NEAR * HIT_RADIUS_NEAR;
        if !(near_now || segment_intersects_circle(a, b, player_pos, HIT_RADIUS)) {
            continue;
        }

        health.current = (health.current - 5).max(0);
        commands
            .entity(player_entity)
            .insert(HurtShake::on_damage_tick());
        fly.hit_cooldown = 0.2;

        if health.current == 0 {
            health.dead = true;
            died_writer.send(PlayerDied);
            return;
        }
    }
}

pub fn ghost_sprite_system(
    mut ghosts: Query<
        (&Transform, &mut Handle<Image>, &GhostAssets, &GhostMotion),
        (With<Ghost>, Without<Player>),
    >,
) {
    for (tf, mut texture, assets, motion) in &mut ghosts {
        let cur = tf.translation.truncate();
        let dx = cur.x - motion.last_pos.x;
        if dx > 0.5 {
            *texture = assets.right.clone();
        } else {
            *texture = assets.idle_or_left.clone();
        }
    }
}
