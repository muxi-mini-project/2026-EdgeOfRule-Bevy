pub mod player;
pub mod sewage_damage;
pub mod water;

use crate::core::state::GameState;
use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player::player_control_system)
            .add_systems(Update, player::player_ground_detection)
            .add_systems(Update, water::water_intersection_detection)
            .add_systems(Update, water::water_physics_system)
            .add_systems(Update, water::reset_gravity_when_not_in_water)
            .add_systems(Update, sewage_damage::sewage_damage_system)
            .add_systems(Update, sewage_damage::on_player_died)
            .add_systems(
                OnEnter(GameState::Day1Scene1),
                sewage_damage::reset_health_on_scene1_enter,
            );
    }
}
