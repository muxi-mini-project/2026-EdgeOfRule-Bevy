mod arrow;
mod door;
mod player;
mod trapdoor;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player::player_animation_system)
            .add_systems(Update, arrow::arrow_animation_system)
            .add_systems(Update, door::door_animation_system)
            .add_systems(Update, trapdoor::trapdoor_animation_system);
    }
}
