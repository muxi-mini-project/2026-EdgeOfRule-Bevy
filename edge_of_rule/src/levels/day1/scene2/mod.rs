pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct Scene2Plugin;

impl Plugin for Scene2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day1Scene2),
            (
                spawner::background::spawn,
                spawner::ground_and_wall::spawn,
                spawner::door::spawn,
                spawner::player::spawn,
                spawner::trapdoor::spawn,
                spawner::mirror::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day1Scene2),
            (
                spawner::background::despawn,
                spawner::ground_and_wall::despawn,
                spawner::door::despawn,
                spawner::player::despawn,
                spawner::trapdoor::despawn,
                spawner::mirror::despawn,
                spawner::arrow_of_door::despawn_all,
                spawner::press_e_to_back_scene1::despawn_all,
            ),
        )
        .add_systems(
            Update,
            (
                spawner::arrow_of_door::spawn.run_if(in_state(GameState::Day1Scene2)),
                spawner::arrow_of_door::despawn,
                spawner::press_e_to_back_scene1::spawn.run_if(in_state(GameState::Day1Scene2)),
                spawner::press_e_to_back_scene1::despawn,
            ),
        )
        .add_systems(
            Update,
            actions::back_to_scene1.run_if(in_state(GameState::Day1Scene2)),
        );
    }
}
