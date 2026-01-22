pub mod day1;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day1),
            (
                day1::spawner::background::spawn,
                day1::spawner::player::spawn,
                day1::spawner::ground_and_wall::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Day1),
            (
                day1::spawner::background::despawn,
                day1::spawner::player::despawn,
                day1::spawner::ground_and_wall::despawn,
            ),
        );
    }
}
