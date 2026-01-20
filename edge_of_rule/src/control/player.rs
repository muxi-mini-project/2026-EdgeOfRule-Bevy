use crate::entities::{ground::Ground, player::Player};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player)>,
) {
    const HORIZONTAL_ACCELERATION: f32 = 3000.0;

    for (mut velocity, mut player) in &mut query {
        let was_dashing = player.is_dashing;
        if handle_active_dash(&time, &mut player, &mut velocity) {
            continue;
        }

        if !was_dashing {
            update_dash_cooldown(&time, &mut player);
        }

        let input = DashInput::from_input(&keyboard_input);
        let dash_ready = player.dash_cooldown_timer.finished();

        if try_trigger_dash(&input, dash_ready, &mut player, &mut velocity) {
            continue;
        }

        apply_horizontal_movement(&time, &player, &mut velocity, input.horizontal_direction);
    }
}

struct DashInput {
    shift_pressed: bool,
    shift_just_pressed: bool,
    a_pressed: bool,
    a_just_pressed: bool,
    d_pressed: bool,
    d_just_pressed: bool,
    s_pressed: bool,
    s_just_pressed: bool,
    horizontal_direction: f32,
}

impl DashInput {
    fn from_input(input: &ButtonInput<KeyCode>) -> Self {
        let shift_pressed = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
        let shift_just_pressed =
            input.just_pressed(KeyCode::ShiftLeft) || input.just_pressed(KeyCode::ShiftRight);
        let a_pressed = input.pressed(KeyCode::KeyA);
        let a_just_pressed = input.just_pressed(KeyCode::KeyA);
        let d_pressed = input.pressed(KeyCode::KeyD);
        let d_just_pressed = input.just_pressed(KeyCode::KeyD);
        let s_pressed = input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown);
        let s_just_pressed =
            input.just_pressed(KeyCode::KeyS) || input.just_pressed(KeyCode::ArrowDown);

        let mut horizontal_direction = 0.0;
        if a_pressed || input.pressed(KeyCode::ArrowLeft) {
            horizontal_direction = -1.0;
        }
        if d_pressed || input.pressed(KeyCode::ArrowRight) {
            horizontal_direction = 1.0;
        }

        Self {
            shift_pressed,
            shift_just_pressed,
            a_pressed,
            a_just_pressed,
            d_pressed,
            d_just_pressed,
            s_pressed,
            s_just_pressed,
            horizontal_direction,
        }
    }

    fn shift_left_combo(&self) -> bool {
        (self.shift_pressed && self.a_just_pressed) || (self.a_pressed && self.shift_just_pressed)
    }

    fn shift_right_combo(&self) -> bool {
        (self.shift_pressed && self.d_just_pressed) || (self.d_pressed && self.shift_just_pressed)
    }

    fn left_combo(&self) -> bool {
        (self.s_pressed && self.a_just_pressed) || (self.a_pressed && self.s_just_pressed)
    }

    fn right_combo(&self) -> bool {
        (self.s_pressed && self.d_just_pressed) || (self.d_pressed && self.s_just_pressed)
    }
}

fn handle_active_dash(time: &Time, player: &mut Player, velocity: &mut Velocity) -> bool {
    if player.is_dashing {
        player.dash_timer.tick(time.delta());
        velocity.linvel.x = player.dash_direction.x * player.dash_speed;
        if player.dash_direction.y != 0.0 {
            velocity.linvel.y = player.dash_direction.y * player.dash_speed;
        }

        if player.dash_timer.finished() {
            player.is_dashing = false;
            player.dash_direction = Vec2::ZERO;
            player.dash_cooldown_timer.reset();
        } else {
            return true;
        }
    }

    false
}

fn update_dash_cooldown(time: &Time, player: &mut Player) {
    if !player.dash_cooldown_timer.finished() {
        player.dash_cooldown_timer.tick(time.delta());
    }
}

fn try_trigger_dash(
    input: &DashInput,
    dash_ready: bool,
    player: &mut Player,
    velocity: &mut Velocity,
) -> bool {
    if !dash_ready {
        return false;
    }

    if input.shift_left_combo() {
        return start_dash(player, velocity, Vec2::new(-1.0, 0.0));
    }

    if input.shift_right_combo() {
        return start_dash(player, velocity, Vec2::new(1.0, 0.0));
    }

    if input.left_combo() {
        let direction = if player.is_grounded {
            Vec2::new(-1.0, 0.0)
        } else {
            Vec2::new(-1.0, -1.0)
        };
        return start_dash(player, velocity, direction);
    }

    if input.right_combo() {
        let direction = if player.is_grounded {
            Vec2::new(1.0, 0.0)
        } else {
            Vec2::new(1.0, -1.0)
        };
        return start_dash(player, velocity, direction);
    }

    false
}

fn start_dash(player: &mut Player, velocity: &mut Velocity, direction: Vec2) -> bool {
    player.is_dashing = true;
    player.dash_direction = direction;
    player.dash_timer.reset();

    velocity.linvel.x = direction.x * player.dash_speed;
    if direction.y != 0.0 {
        velocity.linvel.y = direction.y * player.dash_speed;
    }

    true
}

fn apply_horizontal_movement(
    time: &Time,
    player: &Player,
    velocity: &mut Velocity,
    direction: f32,
) {
    const HORIZONTAL_ACCELERATION: f32 = 3000.0;

    let target_speed = direction * player.speed;
    let delta_speed = target_speed - velocity.linvel.x;
    let max_change = HORIZONTAL_ACCELERATION * time.delta_seconds();
    let change = delta_speed.clamp(-max_change, max_change);
    velocity.linvel.x += change;

    if direction == 0.0 && velocity.linvel.x.abs() < 1.0 {
        velocity.linvel.x = 0.0;
    }
}

pub fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player)>,
) {
    for (mut velocity, mut player) in &mut query {
        if (keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::KeyW))
            && player.jump_count < player.max_jumps
        {
            velocity.linvel.y = player.jump_force;
            player.jump_count += 1;
            player.is_grounded = false;
        }
    }
}

pub fn player_squat(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    const FAST_FALL_ACCELERATION: f32 = 70.0;

    for (mut velocity, player) in &mut query {
        if player.is_dashing {
            continue;
        }

        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.linvel.y -= FAST_FALL_ACCELERATION;
        }
    }
}

pub fn player_ground_detection(
    mut player_query: Query<&mut Player>,
    mut contact_events: EventReader<CollisionEvent>,
    ground_query: Query<Entity, With<Ground>>,
) {
    let ground_entities: Vec<Entity> = ground_query.iter().collect();

    for event in contact_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                for mut player in &mut player_query {
                    if ground_entities.contains(entity1) || ground_entities.contains(entity2) {
                        player.is_grounded = true;
                        player.jump_count = 0;
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                for mut player in &mut player_query {
                    if ground_entities.contains(entity1) || ground_entities.contains(entity2) {
                        player.is_grounded = false;
                    }
                }
            }
        }
    }
}
