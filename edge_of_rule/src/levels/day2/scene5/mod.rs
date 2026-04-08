pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::{core::state::GameState, levels::day2::scene3::ExitOn};

pub struct Scene5Plugin;

impl Plugin for Scene5Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day2Scene5),
            (
                spawner::background::spawn,
                spawner::ground_and_wall::spawn,
                spawner::player::spawn,
                spawner::lift::spawn,
                spawner::hole::spawn,
                actions::reset_lift_door_on_enter
                    .run_if(|exit_on: Res<ExitOn>| *exit_on == ExitOn::Hole),
                actions::start_close_lift_on_enter
                    .run_if(|exit_on: Res<ExitOn>| *exit_on == ExitOn::Lift),
            ),
        )
        .add_systems(
            OnExit(GameState::Day2Scene5),
            (
                spawner::background::despawn,
                spawner::ground_and_wall::despawn,
                spawner::player::despawn,
                spawner::lift::despawn,
                spawner::hole::despawn,
            ),
        )
        .add_systems(
            Update,
            (
                spawner::notice_of_lift::spawn.run_if(in_state(GameState::Day2Scene5)),
                spawner::notice_of_lift::despawn.run_if(in_state(GameState::Day2Scene5)),
                spawner::notice_of_hole::spawn.run_if(in_state(GameState::Day2Scene5)),
                spawner::notice_of_hole::despawn.run_if(in_state(GameState::Day2Scene5)),
                actions::enter_lift.run_if(in_state(GameState::Day2Scene5)),
                actions::back_to_scene4.run_if(in_state(GameState::Day2Scene5)),
                crate::animation::lift_door::tick_lift_door_anim
                    .run_if(in_state(GameState::Day2Scene5)),
            ),
        );

        app.add_systems(
            OnExit(GameState::Day2Scene5),
            (
                spawner::notice_of_lift::despawn_all,
                spawner::notice_of_hole::despawn_all,
            ),
        );
    }
}
