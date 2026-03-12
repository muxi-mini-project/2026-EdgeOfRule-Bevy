pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct Scene3Plugin;

impl Plugin for Scene3Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day1Scene3),
            (
                spawner::background::spawn,
                spawner::ladder::spawn,
                spawner::platform::spawn,
                spawner::pipe::spawn,
                spawner::ground_and_wall::spawn,
                spawner::player::spawn,
                spawner::door::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day1Scene3),
            (
                spawner::background::despawn,
                spawner::ladder::despawn,
                spawner::platform::despawn,
                spawner::pipe::despawn,
                spawner::ground_and_wall::despawn,
                spawner::player::despawn,
                spawner::door::despawn,
                spawner::arrow_of_door::despawn_all,
                spawner::press_e_to_open_door::despawn_all,
            ),
        )
        .add_systems(
            Update,
            (
                spawner::arrow_of_door::spawn.run_if(in_state(GameState::Day1Scene3)),
                spawner::arrow_of_door::despawn,
                spawner::press_e_to_open_door::spawn.run_if(in_state(GameState::Day1Scene3)),
                spawner::press_e_to_open_door::despawn,
            ),
        )
        .add_systems(
            Update,
            (
                actions::open_door.run_if(in_state(GameState::Day1Scene3)),
                actions::enter_door.run_if(in_state(GameState::Day1Scene3)),
            ),
        );
    }
}
