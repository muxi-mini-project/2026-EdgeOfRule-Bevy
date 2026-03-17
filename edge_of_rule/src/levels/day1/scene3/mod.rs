pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

#[derive(Resource, Eq, PartialEq)]
pub enum Scene3DoorState {
    Closed,
    Opened,
}

#[derive(Resource, Eq, PartialEq)]
pub enum Scene3ChestState {
    Packed,
    Opened,
    Picked,
}

#[derive(Resource, Eq, PartialEq)]
pub enum Scene3CoverState {
    Packed,
    Opened,
    Picked,
}

pub struct Scene3Plugin;

impl Plugin for Scene3Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scene3DoorState::Closed)
            .insert_resource(Scene3ChestState::Packed)
            .insert_resource(Scene3CoverState::Packed)
            .add_systems(
                OnEnter(GameState::Day1Scene3),
                (
                    spawner::background::spawn,
                    spawner::ladder::spawn,
                    spawner::platform::spawn,
                    spawner::pipe::spawn,
                    spawner::ground_and_wall::spawn,
                    spawner::player::spawn,
                    spawner::door::spawn,
                    spawner::fog::spawn,
                    spawner::chest::spawn,
                    spawner::elevator::spawn,
                    spawner::hole::spawn,
                    spawner::cover::spawn,
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
                    spawner::fog::despawn,
                    spawner::chest::despawn,
                    spawner::elevator::despawn,
                    spawner::hole::despawn,
                    spawner::cover::despawn,
                    spawner::arrow_of_door::despawn_all,
                    spawner::press_e_to_open_door::despawn_all,
                    spawner::arrow_of_chest::despawn_all,
                    spawner::press_e_to_open_chest::despawn_all,
                    spawner::arrow_of_hole::despawn_all,
                    spawner::press_e_to_enter_hole::despawn_all,
                    spawner::arrow_of_cover::despawn_all,
                    spawner::press_e_to_open_cover::despawn_all,
                ),
            )
            .add_systems(
                Update,
                (
                    spawner::arrow_of_door::spawn.run_if(in_state(GameState::Day1Scene3)),
                    spawner::arrow_of_door::despawn,
                    spawner::press_e_to_open_door::spawn.run_if(in_state(GameState::Day1Scene3)),
                    spawner::press_e_to_open_door::despawn,
                    spawner::arrow_of_chest::spawn.run_if(in_state(GameState::Day1Scene3)),
                    spawner::arrow_of_chest::despawn,
                    spawner::press_e_to_open_chest::spawn.run_if(in_state(GameState::Day1Scene3)),
                    spawner::press_e_to_open_chest::despawn,
                    spawner::arrow_of_hole::spawn.run_if(in_state(GameState::Day1Scene3)),
                    spawner::arrow_of_hole::despawn,
                    spawner::press_e_to_enter_hole::spawn.run_if(in_state(GameState::Day1Scene3)),
                    spawner::press_e_to_enter_hole::despawn,
                    spawner::arrow_of_cover::spawn.run_if(in_state(GameState::Day1Scene3)),
                    spawner::arrow_of_cover::despawn,
                    spawner::press_e_to_open_cover::spawn.run_if(in_state(GameState::Day1Scene3)),
                    spawner::press_e_to_open_cover::despawn,
                ),
            )
            .add_systems(
                Update,
                (
                    actions::open_door.run_if(in_state(GameState::Day1Scene3)),
                    actions::enter_door.run_if(in_state(GameState::Day1Scene3)),
                    actions::enter_hole.run_if(in_state(GameState::Day1Scene3)),
                    actions::fog_follow,
                    actions::open_chest,
                    actions::open_cover,
                    actions::move_elevator_when_chest_picked
                        .run_if(in_state(GameState::Day1Scene3)),
                ),
            );
    }
}
