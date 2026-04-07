pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::{
    core::state::GameState,
    levels::day2::scene3::actions::{Buttons, LiftOpenAnim, LiftState},
};

pub struct Scene3Plugin;

impl Plugin for Scene3Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Buttons::default())
            .insert_resource(LiftState::Broken)
            .insert_resource(LiftOpenAnim::default())
            .add_systems(
                OnEnter(GameState::Day2Scene3),
                (
                    spawner::background::spawn,
                    spawner::ground_and_wall::spawn,
                    spawner::player::spawn,
                    spawner::mask::spawn,
                    spawner::hole::spawn,
                    spawner::lift::spawn,
                    spawner::curcuit::spawn,
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
                    spawner::lift::despawn,
                    spawner::curcuit::despawn,
                ),
            )
            .add_systems(
                Update,
                (
                    spawner::notice_of_hole::spawn.run_if(in_state(GameState::Day2Scene3)),
                    spawner::notice_of_hole::despawn.run_if(in_state(GameState::Day2Scene3)),
                    spawner::notice_of_circuit::spawn.run_if(in_state(GameState::Day2Scene3)),
                    spawner::notice_of_circuit::despawn.run_if(in_state(GameState::Day2Scene3)),
                    spawner::notice_of_lift::spawn.run_if(in_state(GameState::Day2Scene3)),
                    spawner::notice_of_lift::despawn.run_if(in_state(GameState::Day2Scene3)),
                ),
            )
            .add_systems(
                OnExit(GameState::Day2Scene3),
                (
                    spawner::notice_of_hole::despawn_all,
                    spawner::notice_of_circuit::despawn_all,
                    spawner::notice_of_lift::despawn_all,
                ),
            )
            .add_systems(
                Update,
                (
                    actions::enter_scene1.run_if(in_state(GameState::Day2Scene3)),
                    actions::enter_hole.run_if(in_state(GameState::Day2Scene3)),
                    actions::check_circuit.run_if(in_state(GameState::Day2Scene3)),
                    actions::close_circuit.run_if(in_state(GameState::Day2Scene3)),
                    actions::toggle_button.run_if(in_state(GameState::Day2Scene3)),
                    actions::update_lift.run_if(in_state(GameState::Day2Scene3)),
                    actions::update_button.run_if(in_state(GameState::Day2Scene3)),
                    actions::tick_lift_open_anim.run_if(in_state(GameState::Day2Scene3)),
                    actions::enter_lift.run_if(in_state(GameState::Day2Scene3)),
                ),
            );
    }
}
