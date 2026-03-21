use bevy::prelude::*;
use crate::core::state::GameState;
use crate::ui::hud::spawner::ingame_option_area::BackToMainMenuBtn;

pub fn on_click(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackToMainMenuBtn>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            state.set(GameState::MainMenu);
        }
    }
}   

