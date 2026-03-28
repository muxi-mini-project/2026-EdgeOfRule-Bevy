use bevy::prelude::*;
use bevy::sprite::Anchor; 

use crate::assets::hud::HudImageAssets;
use crate::core::health::{PlayerHealth, PlayerDied, SewageDamageAccum};

use crate::ui::hud::spawner::blood_bar::{BloodBar, BloodFill};

pub fn update_blood_bar(
    mut blood_bars: Query<&mut Transform, With<BloodBar>>,
    player_health: Res<PlayerHealth>,
) {
    let health_percent = player_health.current as f32 / player_health.max as f32;
    
    for mut transform in blood_bars.iter_mut() {
        transform.scale.x = health_percent;
    }
}