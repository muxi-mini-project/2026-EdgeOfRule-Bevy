pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::{core::state::GameState, entities::player::SpawnPoint};

const DEFAULT_SPAWN: Vec3 = Vec3::new(0.0, -68.0, 0.0);

pub struct Scene2Plugin;

impl Plugin for Scene2Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnPoint(Transform::from_translation(DEFAULT_SPAWN)))
            .add_systems(
                OnEnter(GameState::Day2Scene2),
                (
                    spawner::background::spawn,
                    spawner::ground_and_wall::spawn,
                    spawner::player::spawn,
                    spawner::door::spawn,
                    spawner::desk::spawn,
                ),
            )
            .add_systems(
                OnExit(GameState::Day2Scene2),
                (
                    spawner::background::despawn,
                    spawner::ground_and_wall::despawn,
                    spawner::player::despawn,
                    spawner::door::despawn,
                    spawner::desk::despawn,
                    spawner::notice_of_door::despawn_all_left,
                    spawner::notice_of_door::despawn_all_right,
                ),
            )
            .add_systems(
                Update,
                (
                    spawner::notice_of_door::spawn.run_if(in_state(GameState::Day2Scene2)),
                    spawner::notice_of_door::despawn.run_if(in_state(GameState::Day2Scene2)),
                    actions::exit_left_door.run_if(in_state(GameState::Day2Scene2)),
                    actions::exit_right_door.run_if(in_state(GameState::Day2Scene2)),
                ),
            );
    }
}
