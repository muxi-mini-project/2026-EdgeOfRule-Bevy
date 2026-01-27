mod arrow;
mod player;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player::player_animation_system)
            .add_systems(Update, arrow::arrow_animation_system);
    }
}
