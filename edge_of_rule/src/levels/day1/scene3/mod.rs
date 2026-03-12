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
            ),
        );
    }
}
