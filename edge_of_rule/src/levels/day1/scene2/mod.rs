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
            ),
        );
    }
}
