use bevy::prelude::*;

use crate::{core::state::GameState, ui::mainmenu::spawner::backward_btn::BackwardBtn};

pub fn on_click(
    mut next_state: ResMut<NextState<GameState>>,
    btns: Query<&Interaction, With<BackwardBtn>>,
) {
    for btn in &btns {
        if *btn == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}
