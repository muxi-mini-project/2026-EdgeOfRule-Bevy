pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct Scene4Plugin;

impl Plugin for Scene4Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day1Scene4),
            (
                spawner::background::spawn,
                spawner::ground_and_wall::spawn,
                spawner::player::spawn,
                spawner::fog::spawn,
                spawner::log::spawn,
                spawner::door::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day1Scene4),
            (
                spawner::background::despawn,
                spawner::ground_and_wall::despawn,
                spawner::player::despawn,
                spawner::fog::despawn,
                spawner::log::despawn,
                spawner::door::despawn,
                spawner::arrow_of_door::despawn_all,
                spawner::press_e_to_enter::despawn_all,
                spawner::arrow_of_log::despawn_all,
                spawner::press_e_to_read_log::despawn_all,
            ),
        )
        .add_systems(
            Update,
            (
                spawner::arrow_of_door::spawn.run_if(in_state(GameState::Day1Scene4)),
                spawner::arrow_of_door::despawn,
                spawner::press_e_to_enter::spawn.run_if(in_state(GameState::Day1Scene4)),
                spawner::press_e_to_enter::despawn,
                spawner::arrow_of_log::spawn.run_if(in_state(GameState::Day1Scene4)),
                spawner::arrow_of_log::despawn,
                spawner::press_e_to_read_log::spawn.run_if(in_state(GameState::Day1Scene4)),
                spawner::press_e_to_read_log::despawn,
            ),
        )
        .add_systems(
            Update,
            (
                actions::fog_follow,
                actions::back_to_scene3,
                actions::read_log,
                actions::close_log,
            ),
        );
    }
}
