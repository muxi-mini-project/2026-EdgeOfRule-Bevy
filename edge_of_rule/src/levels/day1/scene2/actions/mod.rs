use bevy::prelude::*;

use crate::{
    core::state::GameState,
    levels::day1::scene2::spawner::{
        press_e_to_back_scene1::PressEtoBackScene1,
        press_e_to_enter_trapdoor::PressEtoEnterTrapdoor,
    },
};

pub fn back_to_scene1(
    query: Query<&PressEtoBackScene1>,
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        state.set(GameState::Day1Scene1);
    }
}

pub fn enter_trapdoor(
    query: Query<&PressEtoEnterTrapdoor>,
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        state.set(GameState::Day2);
    }
}
