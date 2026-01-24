use bevy::prelude::*;

use crate::{core::state::GameState, ui::mainmenu::spawner::options_btn::OptionBtn};

pub fn on_click(
    mut next_state: ResMut<NextState<GameState>>,
    btns: Query<&Interaction, With<OptionBtn>>,
) {
    for btn in &btns {
        if *btn == Interaction::Pressed {
            next_state.set(GameState::Day1Scene1);
        }
    }
}
