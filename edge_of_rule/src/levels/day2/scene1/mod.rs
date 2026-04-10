pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct Scene1Plugin;

impl Plugin for Scene1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day2Scene1),
            (
                spawner::background::spawn,
                spawner::ground_and_wall::spawn,
                spawner::player::spawn,
                spawner::mask::spawn,
                spawner::notice_board::spawn,
                spawner::door::spawn,
                spawner::windows::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day2Scene1),
            (
                spawner::background::despawn,
                spawner::ground_and_wall::despawn,
                spawner::player::despawn,
                spawner::mask::despawn,
                spawner::notice_board::despawn,
                spawner::door::despawn,
                spawner::windows::despawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day2Scene1),
            spawner::notice_of_notice_board::despawn_all,
        )
        .add_systems(
            OnExit(GameState::Day2Scene1),
            spawner::notice_of_windows::despawn_all,
        )
        .add_systems(
            Update,
            (
                spawner::notice_of_notice_board::spawn,
                spawner::notice_of_notice_board::despawn,
            ),
        )
        .add_systems(
            Update,
            (
                spawner::notice_of_windows::spawn.run_if(in_state(GameState::Day2Scene1)),
                spawner::notice_of_windows::despawn.run_if(in_state(GameState::Day2Scene1)),
            ),
        )
        .add_systems(
            Update,
            (
                actions::read_small_note,
                actions::close_small_note,
                actions::enter_scene3,
                actions::enter_scene2,
            ),
        );
    }
}
