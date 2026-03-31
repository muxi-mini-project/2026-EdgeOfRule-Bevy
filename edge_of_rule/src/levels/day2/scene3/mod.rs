pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct Scene3Plugin;

impl Plugin for Scene3Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day2Scene3),
            (
                spawner::background::spawn,
                spawner::ground_and_wall::spawn,
                spawner::player::spawn,
                spawner::mask::spawn,
                spawner::hole::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day2Scene3),
            (
                spawner::background::despawn,
                spawner::ground_and_wall::despawn,
                spawner::player::despawn,
                spawner::mask::despawn,
                spawner::hole::despawn,
            ),
        )
        .add_systems(
            Update,
            (
                spawner::notice_of_hole::spawn.run_if(in_state(GameState::Day2Scene3)),
                spawner::notice_of_hole::despawn.run_if(in_state(GameState::Day2Scene3)),
            ),
        )
        .add_systems(
            OnExit(GameState::Day2Scene3),
            spawner::notice_of_hole::despawn_all,
        )
        .add_systems(
            Update,
            (
                actions::enter_scene1.run_if(in_state(GameState::Day2Scene3)),
                actions::enter_hole.run_if(in_state(GameState::Day2Scene3)),
            ),
        );
    }
}
