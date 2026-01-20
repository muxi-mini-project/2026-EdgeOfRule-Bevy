pub mod player;

use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player::player_movement)
            .add_systems(Update, player::player_jump)
            .add_systems(Update, player::player_squat)
            .add_systems(Update, player::player_ground_detection);
    }
}
