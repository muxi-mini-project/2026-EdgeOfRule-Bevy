pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

#[derive(Resource, Eq, PartialEq)]
pub enum Picked {
    None,
    Key,
    Screw,
}

pub struct Scene1Plugin;

impl Plugin for Scene1Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Picked::None)
            .add_systems(
                OnEnter(GameState::Day1Scene1),
                (
                    spawner::background::spawn,
                    spawner::ground_and_wall::spawn,
                    spawner::player::spawn,
                    spawner::table::spawn,
                    spawner::broken_floor::spawn,
                    spawner::door::spawn,
                    spawner::small_note::spawn,
                    spawner::key::spawn,
                    spawner::screw::spawn,
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
                    spawner::small_note::despawn,
                    spawner::arrow_of_door::despawn_all,
                    spawner::press_e_to_open_door::despawn_all,
                    spawner::arrow_of_small_note::despawn_all,
                    spawner::press_e_to_read::despawn_all,
                    spawner::arrow_of_key::despawn_all,
                    spawner::press_e_to_pick_key::despawn_all,
                    spawner::arrow_of_screw::despawn_all,
                    spawner::press_e_to_pick_screw::despawn_all,
                    spawner::arrow_of_return_key::despawn_all,
                    spawner::press_e_to_return_key::despawn_all,
                    spawner::key::despawn,
                    spawner::screw::despawn,
                ),
            )
            .add_systems(
                OnExit(GameState::Day1Scene1),
                (
                    spawner::arrow_of_return_screw::despawn_all,
                    spawner::press_e_to_return_screw::despawn_all,
                ),
            )
            .add_systems(
                Update,
                (
                    spawner::arrow_of_door::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::arrow_of_door::despawn,
                    spawner::press_e_to_open_door::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::press_e_to_open_door::despawn,
                    spawner::arrow_of_small_note::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::arrow_of_small_note::despawn,
                    spawner::press_e_to_read::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::press_e_to_read::despawn,
                    spawner::arrow_of_key::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::arrow_of_key::despawn,
                    spawner::press_e_to_pick_key::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::press_e_to_pick_key::despawn,
                    spawner::arrow_of_screw::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::arrow_of_screw::despawn,
                    spawner::press_e_to_pick_screw::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::press_e_to_pick_screw::despawn,
                    spawner::arrow_of_return_key::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::arrow_of_return_key::despawn,
                    spawner::press_e_to_return_key::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::press_e_to_return_key::despawn,
                ),
            )
            .add_systems(
                Update,
                (
                    spawner::arrow_of_return_screw::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::arrow_of_return_screw::despawn,
                    spawner::press_e_to_return_screw::spawn.run_if(in_state(GameState::Day1Scene1)),
                    spawner::press_e_to_return_screw::despawn,
                ),
            )
            .add_systems(
                Update,
                (
                    actions::open_door.run_if(in_state(GameState::Day1Scene1)),
                    actions::read_small_note.run_if(in_state(GameState::Day1Scene1)),
                    actions::close_small_note.run_if(in_state(GameState::Day1Scene1)),
                    actions::pick_key.run_if(in_state(GameState::Day1Scene1)),
                    actions::pick_screw.run_if(in_state(GameState::Day1Scene1)),
                    actions::return_key.run_if(in_state(GameState::Day1Scene1)),
                    actions::return_screw.run_if(in_state(GameState::Day1Scene1)),
                ),
            );
    }
}
