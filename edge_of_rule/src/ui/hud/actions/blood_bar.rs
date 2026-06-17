use bevy::prelude::*;

use crate::core::health::PlayerHealth;

use crate::ui::hud::spawner::blood_bar::BloodBar;

pub fn update_blood_bar(
    mut blood_bars: Query<&mut Style, With<BloodBar>>,
    player_health: Res<PlayerHealth>,
) {
    let health_percent = player_health.current as f32 / player_health.max as f32;

    for mut style in blood_bars.iter_mut() {
        style.width = Val::Px(200.0 * health_percent);
    }
}
