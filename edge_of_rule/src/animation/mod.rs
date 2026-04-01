mod arrow;
mod chest;
mod cover;
mod door;
mod elevator;
pub mod fade_mask;
pub mod hurt_shake;
mod lift;
mod player;
mod trapdoor;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player::player_animation_system)
            .add_systems(Update, arrow::arrow_animation_system)
            .add_systems(Update, hurt_shake::hurt_shake_system)
            .add_systems(Update, fade_mask::fade_animation)
            .add_systems(Update, elevator::elevator_animation_system)
            .add_systems(PostUpdate, lift::lift_animation_system)
            .add_systems(
                Update,
                door::scene1_door_animation_system.run_if(in_state(GameState::Day1Scene1)),
            )
            .add_systems(
                Update,
                door::scene1_door_animation_system.run_if(in_state(GameState::Day1Scene2)),
            )
            .add_systems(
                Update,
                door::scene3_door_animation_system.run_if(in_state(GameState::Day1Scene3)),
            )
            .add_systems(
                Update,
                door::scene3_door_animation_system.run_if(in_state(GameState::Day1Scene4)),
            )
            .add_systems(Update, trapdoor::trapdoor_animation_system)
            .add_systems(
                Update,
                chest::chest_animation_system.run_if(in_state(GameState::Day1Scene3)),
            )
            .add_systems(
                Update,
                cover::cover_animation_system.run_if(in_state(GameState::Day1Scene3)),
            );
    }
}
