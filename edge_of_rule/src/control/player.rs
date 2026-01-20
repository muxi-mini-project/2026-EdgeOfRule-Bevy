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
        if player.is_dashing {
            player.dash_timer.tick(time.delta());
            velocity.linvel.x = player.dash_direction * player.dash_speed;

            if player.dash_timer.finished() {
                player.is_dashing = false;
                player.dash_direction = 0.0;
                player.dash_cooldown_timer.reset();
            } else {
                continue;
            }
        } else if !player.dash_cooldown_timer.finished() {
            player.dash_cooldown_timer.tick(time.delta());
        }

        let dash_ready = player.dash_cooldown_timer.finished();
        let shift_just_pressed = keyboard_input.just_pressed(KeyCode::ShiftLeft)
            || keyboard_input.just_pressed(KeyCode::ShiftRight);

        if keyboard_input.pressed(KeyCode::KeyA) && shift_just_pressed && dash_ready {
            player.is_dashing = true;
            player.dash_direction = -1.0;
            player.dash_timer.reset();
            velocity.linvel.x = player.dash_direction * player.dash_speed;
            continue;
        }
        if keyboard_input.pressed(KeyCode::KeyD) && shift_just_pressed && dash_ready {
            player.is_dashing = true;
            player.dash_direction = 1.0;
            player.dash_timer.reset();
            velocity.linvel.x = player.dash_direction * player.dash_speed;
            continue;
        }

        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction = -1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction = 1.0;
        }

        let target_speed = direction * player.speed;
        let delta_speed = target_speed - velocity.linvel.x;
        let max_change = HORIZONTAL_ACCELERATION * time.delta_seconds();
        let change = delta_speed.clamp(-max_change, max_change);
        velocity.linvel.x += change;

        if direction == 0.0 && velocity.linvel.x.abs() < 1.0 {
            velocity.linvel.x = 0.0;
        }
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
