pub mod player;

use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player::player_control_system)
            .add_systems(Update, player::player_ground_detection);
    }
}
