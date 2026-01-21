use crate::entities::{
    ground::Ground,
    player::{FacingDirection, Player, PlayerState},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const FAST_FALL_ACCELERATION: f32 = 70.0;

pub fn player_control_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player)>,
) {
    for (mut velocity, mut player) in &mut query {
        let input = PlayerInput::from(&keyboard_input);

        if handle_active_dash(&time, &mut player, &mut velocity) {
            continue;
        }

        update_dash_cooldown(&time, &mut player);

        if try_trigger_dash(&input, &mut player, &mut velocity) {
            continue;
        }

        handle_jump(&input, &mut player, &mut velocity);

        let horizontal_input = input.horizontal_direction();
        let down_without_move = player.is_grounded && input.down_pressed && horizontal_input == 0.0;
        let horizontal_direction = if down_without_move {
            0.0
        } else {
            horizontal_input
        };

        if horizontal_direction != 0.0 {
            player.facing = if horizontal_direction > 0.0 {
                FacingDirection::Right
            } else {
                FacingDirection::Left
            };
        }

        apply_horizontal_movement(&time, &player, &mut velocity, horizontal_direction);

        if player.is_grounded {
            update_ground_state(&mut player, horizontal_input, down_without_move);
        } else {
            update_air_state(&input, &mut player, &mut velocity);
        }
    }
}

struct PlayerInput {
    left_pressed: bool,
    left_just_pressed: bool,
    right_pressed: bool,
    right_just_pressed: bool,
    down_pressed: bool,
    down_just_pressed: bool,
    jump_just_pressed: bool,
    shift_pressed: bool,
    shift_just_pressed: bool,
}

impl PlayerInput {
    fn from(input: &ButtonInput<KeyCode>) -> Self {
        let left_pressed = input.pressed(KeyCode::KeyA);
        let right_pressed = input.pressed(KeyCode::KeyD);
        let down_pressed = input.pressed(KeyCode::KeyS);
        let shift_pressed = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);

        let left_just_pressed = input.just_pressed(KeyCode::KeyA);
        let right_just_pressed = input.just_pressed(KeyCode::KeyD);
        let down_just_pressed = input.just_pressed(KeyCode::KeyS);
        let shift_just_pressed =
            input.just_pressed(KeyCode::ShiftLeft) || input.just_pressed(KeyCode::ShiftRight);

        let jump_just_pressed =
            input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::KeyW);

        Self {
            left_pressed,
            left_just_pressed,
            right_pressed,
            right_just_pressed,
            down_pressed,
            down_just_pressed,
            jump_just_pressed,
            shift_pressed,
            shift_just_pressed,
        }
    }

    fn horizontal_direction(&self) -> f32 {
        match (self.left_pressed, self.right_pressed) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        }
    }

    fn shift_left_combo(&self) -> bool {
        (self.shift_pressed && self.left_just_pressed)
            || (self.left_pressed && self.shift_just_pressed)
    }

    fn shift_right_combo(&self) -> bool {
        (self.shift_pressed && self.right_just_pressed)
            || (self.right_pressed && self.shift_just_pressed)
    }

    fn down_left_combo(&self) -> bool {
        (self.down_pressed && self.left_just_pressed)
            || (self.left_pressed && self.down_just_pressed)
    }

    fn down_right_combo(&self) -> bool {
        (self.down_pressed && self.right_just_pressed)
            || (self.right_pressed && self.down_just_pressed)
    }
}

fn handle_active_dash(time: &Time, player: &mut Player, velocity: &mut Velocity) -> bool {
    if !matches!(player.state, PlayerState::Dashing | PlayerState::Sliding) {
        return false;
    }

    player.dash_timer.tick(time.delta());
    velocity.linvel.x = player.dash_direction.x * player.dash_speed;
    if player.dash_direction.y != 0.0 {
        velocity.linvel.y = player.dash_direction.y * player.dash_speed;
    }

    if player.dash_timer.finished() {
        player.dash_direction = Vec2::ZERO;
        player.dash_cooldown_timer.reset();
        player.ignore_down_input = false;
        player.state = if player.is_grounded {
            PlayerState::Idle
        } else {
            PlayerState::Falling
        };
    } else {
        return true;
    }

    false
}

fn update_dash_cooldown(time: &Time, player: &mut Player) {
    if !player.dash_cooldown_timer.finished() {
        player.dash_cooldown_timer.tick(time.delta());
    }
}

fn try_trigger_dash(input: &PlayerInput, player: &mut Player, velocity: &mut Velocity) -> bool {
    if !player.dash_cooldown_timer.finished() {
        return false;
    }

    if input.shift_left_combo() {
        return start_dash(player, velocity, PlayerState::Dashing, Vec2::new(-1.0, 0.0));
    }

    if input.shift_right_combo() {
        return start_dash(player, velocity, PlayerState::Dashing, Vec2::new(1.0, 0.0));
    }

    if input.down_left_combo() {
        let desired_state = if player.is_grounded {
            PlayerState::Sliding
        } else {
            PlayerState::Dashing
        };
        let dash_direction = if player.is_grounded {
            Vec2::new(-1.0, 0.0)
        } else {
            Vec2::new(-1.0, -1.0)
        };
        player.ignore_down_input = player.is_grounded;
        return start_dash(player, velocity, desired_state, dash_direction);
    }

    if input.down_right_combo() {
        let desired_state = if player.is_grounded {
            PlayerState::Sliding
        } else {
            PlayerState::Dashing
        };
        let dash_direction = if player.is_grounded {
            Vec2::new(1.0, 0.0)
        } else {
            Vec2::new(1.0, -1.0)
        };
        player.ignore_down_input = player.is_grounded;
        return start_dash(player, velocity, desired_state, dash_direction);
    }

    false
}

fn start_dash(
    player: &mut Player,
    velocity: &mut Velocity,
    next_state: PlayerState,
    direction: Vec2,
) -> bool {
    player.state = next_state;
    player.dash_direction = direction;
    player.dash_timer.reset();

    if direction.x != 0.0 {
        player.facing = if direction.x > 0.0 {
            FacingDirection::Right
        } else {
            FacingDirection::Left
        };
    }

    velocity.linvel.x = direction.x * player.dash_speed;
    if direction.y != 0.0 {
        velocity.linvel.y = direction.y * player.dash_speed;
    }

    if matches!(player.state, PlayerState::Sliding) {
        velocity.linvel.y = 0.0;
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

fn handle_jump(input: &PlayerInput, player: &mut Player, velocity: &mut Velocity) {
    if input.jump_just_pressed && player.jump_count < player.max_jumps {
        velocity.linvel.y = player.jump_force;
        player.jump_count += 1;
        player.is_grounded = false;
        player.state = PlayerState::Jumping;
    }
}

fn update_ground_state(player: &mut Player, horizontal_input: f32, down_without_move: bool) {
    if matches!(player.state, PlayerState::Sliding) {
        return;
    }

    if down_without_move && !player.ignore_down_input {
        player.state = PlayerState::Crouching;
    } else if horizontal_input != 0.0 {
        player.state = PlayerState::Walking;
    } else {
        player.state = PlayerState::Idle;
    }
}

fn update_air_state(input: &PlayerInput, player: &mut Player, velocity: &mut Velocity) {
    if input.down_pressed && !player.ignore_down_input {
        player.state = PlayerState::FastFalling;
        velocity.linvel.y -= FAST_FALL_ACCELERATION;
        return;
    }

    if velocity.linvel.y > 1.0 {
        player.state = PlayerState::Jumping;
    } else {
        player.state = PlayerState::Falling;
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
                if ground_entities.contains(entity1) || ground_entities.contains(entity2) {
                    for mut player in &mut player_query {
                        player.is_grounded = true;
                        player.jump_count = 0;
                        if matches!(
                            player.state,
                            PlayerState::Jumping | PlayerState::Falling | PlayerState::FastFalling
                        ) {
                            player.state = PlayerState::Idle;
                        }
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                if ground_entities.contains(entity1) || ground_entities.contains(entity2) {
                    for mut player in &mut player_query {
                        player.is_grounded = false;
                        if matches!(
                            player.state,
                            PlayerState::Idle | PlayerState::Walking | PlayerState::Crouching
                        ) {
                            player.state = PlayerState::Falling;
                        }
                    }
                }
            }
        }
    }
}
