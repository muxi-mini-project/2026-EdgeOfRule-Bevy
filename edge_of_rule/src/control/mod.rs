pub mod player;
pub mod water;

use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player::player_control_system)
            .add_systems(Update, player::player_ground_detection)
            .add_systems(Update, water::water_intersection_detection)
            .add_systems(Update, water::water_physics_system)
            .add_systems(Update, water::reset_gravity_when_not_in_water);
    }
}
