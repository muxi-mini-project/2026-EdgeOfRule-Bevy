use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::health::PlayerHealth;
use crate::entities::{player::Player, sewage::Sewage};

pub fn water_intersection_detection(
    mut events: EventReader<CollisionEvent>,
    players: Query<Entity, With<Player>>,
    waters: Query<Entity, With<Sewage>>,
    mut player_state: Query<&mut Player>,
) {
    for ev in events.read() {
        let (e1, e2, is_intersecting) = match ev {
            CollisionEvent::Started(e1, e2, _) => (*e1, *e2, true),
            CollisionEvent::Stopped(e1, e2, _) => (*e1, *e2, false),
        };

        let (player_entity, is_player_water_pair) =
            if players.get(e1).is_ok() && waters.get(e2).is_ok() {
                (e1, true)
            } else if players.get(e2).is_ok() && waters.get(e1).is_ok() {
                (e2, true)
            } else {
                (Entity::PLACEHOLDER, false)
            };

        if !is_player_water_pair {
            continue;
        }

        if let Ok(mut player) = player_state.get_mut(player_entity) {
            if is_intersecting {
                player.water_contacts = player.water_contacts.saturating_add(1);
            } else {
                player.water_contacts = player.water_contacts.saturating_sub(1);
            }
        }
    }
}

pub fn water_physics_system(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    health: Res<PlayerHealth>,
    mut players: Query<(&Player, &mut Velocity, &mut GravityScale)>,
) {
    if health.dead {
        return;
    }

    let dt = time.delta_seconds();

    for (player, mut velocity, mut gravity_scale) in &mut players {
        if !player.in_water() {
            continue;
        }

        // Reduced gravity in water.
        gravity_scale.0 = 3.0;

        // Drag (exponential decay).
        // Keep a bit more vertical freedom so up/down feels responsive.
        let drag_x = 7.0;
        let drag_y = 4.0;
        velocity.linvel.x *= (-drag_x * dt).exp();
        velocity.linvel.y *= (-drag_y * dt).exp();

        // Clamp sinking speed.
        velocity.linvel.y = velocity.linvel.y.max(-180.0);

        // Hold Space/W to float up.
        if input.pressed(KeyCode::Space) || input.pressed(KeyCode::KeyW) {
            const FLOAT_ACCEL: f32 = 900.0;
            const MAX_UP_SPEED: f32 = 220.0;
            velocity.linvel.y = (velocity.linvel.y + FLOAT_ACCEL * dt).min(MAX_UP_SPEED);
        }
    }
}

pub fn reset_gravity_when_not_in_water(mut players: Query<(&Player, &mut GravityScale)>) {
    for (player, mut gravity_scale) in &mut players {
        if player.in_water() {
            continue;
        }

        if (gravity_scale.0 - 12.0).abs() > f32::EPSILON {
            gravity_scale.0 = 12.0;
        }
    }
}
