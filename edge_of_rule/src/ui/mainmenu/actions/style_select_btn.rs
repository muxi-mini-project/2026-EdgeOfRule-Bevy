use bevy::prelude::*;

use crate::{core::state::GameState, ui::mainmenu::spawner::style_select_btn::StyleSelectBtn};

pub fn on_click(
    mut next_state: ResMut<NextState<GameState>>,
    btns: Query<&Interaction, With<StyleSelectBtn>>,
) {
    for btn in &btns {
        if *btn == Interaction::Pressed {
            next_state.set(GameState::StyleSelect);
        }
    }
}
