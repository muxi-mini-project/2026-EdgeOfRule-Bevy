use bevy::prelude::*;

use crate::{core::state::GameState, ui::mainmenu::spawner::game_start_btn::GameStartBtn};

pub fn on_click(
    mut next_state: ResMut<NextState<GameState>>,
    btns: Query<&Interaction, With<GameStartBtn>>,
) {
    for btn in &btns {
        if *btn == Interaction::Pressed {
            next_state.set(GameState::LevelSelect);
        }
    }
}
