use bevy::prelude::*;

use crate::{core::health::PlayerHealth, entities::player::PlayerSprite};

#[derive(Component, Debug, Clone)]
pub struct HurtShake {
    pub timer: Timer,
    pub amplitude: f32,
    pub frequency_hz: f32,
    pub phase: f32,
}

impl HurtShake {
    pub fn on_damage_tick() -> Self {
        Self {
            timer: Timer::from_seconds(0.25, TimerMode::Once),
            amplitude: 0.6,
            frequency_hz: 14.0,
            phase: 0.0,
        }
    }
}

pub fn hurt_shake_system(
    mut commands: Commands,
    time: Res<Time>,
    health: Res<PlayerHealth>,
    mut shakers: Query<(Entity, &Children, &mut HurtShake)>,
    mut sprites: Query<&mut Transform, With<PlayerSprite>>,
) {
    let dt = time.delta_seconds();

    for (entity, children, mut shake) in &mut shakers {
        shake.timer.tick(time.delta());
        shake.phase += dt * shake.frequency_hz * std::f32::consts::TAU;

        let should_remove = health.dead || shake.timer.finished();
        let desired_x = if should_remove {
            0.0
        } else {
            shake.amplitude * shake.phase.sin()
        };

        let mut applied = false;
        for &child in children.iter() {
            if let Ok(mut sprite_transform) = sprites.get_mut(child) {
                sprite_transform.translation.x = desired_x;
                applied = true;
                break;
            }
        }

        if !applied || should_remove {
            commands.entity(entity).remove::<HurtShake>();
        }
    }
}
