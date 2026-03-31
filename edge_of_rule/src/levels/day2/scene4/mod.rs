pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct Scene4Plugin;

impl Plugin for Scene4Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day2Scene4),
            (
                spawner::background::spawn,
                spawner::ground_and_wall::spawn,
                spawner::player::spawn,
                spawner::hole::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day2Scene4),
            (
                spawner::background::despawn,
                spawner::ground_and_wall::despawn,
                spawner::player::despawn,
                spawner::hole::despawn,
            ),
        )
        .add_systems(
            Update,
            (
                spawner::notice_of_entrance::spawn.run_if(in_state(GameState::Day2Scene4)),
                spawner::notice_of_entrance::despawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day2Scene4),
            spawner::notice_of_entrance::despawn_all,
        )
        .add_systems(Update, actions::back_to_scene3);
    }
}
