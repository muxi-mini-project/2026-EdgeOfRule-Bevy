use bevy::prelude::*;

use crate::{core::state::GameState, ui::mainmenu::spawner::rules_btn::RulesBtn};

pub fn on_click(
    mut next_state: ResMut<NextState<GameState>>,
    btns: Query<&Interaction, With<RulesBtn>>,
) {
    for btn in &btns {
        if *btn == Interaction::Pressed {
            next_state.set(GameState::RulesView);
        }
    }
}
