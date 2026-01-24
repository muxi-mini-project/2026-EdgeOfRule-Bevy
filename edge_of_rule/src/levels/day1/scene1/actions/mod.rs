use bevy::prelude::*;

use crate::{
    core::state::GameState, levels::day1::scene1::spawner::press_e_to_open_door::PressEtoOpenDoor,
};

pub fn open_door(
    query: Query<&PressEtoOpenDoor>,
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        state.set(GameState::Day1Scene2);
    }
}
