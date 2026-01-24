pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct Scene1Plugin;

impl Plugin for Scene1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day1Scene1),
            (
                spawner::background::spawn,
                spawner::ground_and_wall::spawn,
                spawner::player::spawn,
                spawner::table::spawn,
                spawner::broken_floor::spawn,
                spawner::door::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day1Scene1),
            (
                spawner::background::despawn,
                spawner::ground_and_wall::despawn,
                spawner::player::despawn,
                spawner::table::despawn,
                spawner::broken_floor::despawn,
                spawner::door::despawn,
                spawner::arrow_of_door::despawn_all,
                spawner::press_e_to_open_door::despawn_all,
            ),
        )
        .add_systems(
            Update,
            (
                spawner::arrow_of_door::spawn.run_if(in_state(GameState::Day1Scene1)),
                spawner::arrow_of_door::despawn,
                spawner::press_e_to_open_door::spawn.run_if(in_state(GameState::Day1Scene1)),
                spawner::press_e_to_open_door::despawn,
            ),
        )
        .add_systems(
            Update,
            actions::open_door.run_if(in_state(GameState::Day1Scene1)),
        );
    }
}
