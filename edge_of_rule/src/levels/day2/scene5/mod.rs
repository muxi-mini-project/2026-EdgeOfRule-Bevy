pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct Scene5Plugin;

impl Plugin for Scene5Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Day2Scene5),
            (spawner::lift::spawn, actions::start_close_lift_on_enter),
        )
        .add_systems(OnExit(GameState::Day2Scene5), spawner::lift::despawn)
        .add_systems(
            Update,
            crate::animation::lift_door::tick_lift_door_anim
                .run_if(in_state(GameState::Day2Scene5)),
        );
    }
}
