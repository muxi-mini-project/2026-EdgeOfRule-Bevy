use bevy::prelude::*;

use crate::{core::state::GameState, ui::mainmenu::spawner::style_select_btn::StyleSelectBtn};

pub fn on_click(
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction,&mut BackgroundColor), With<StyleSelectBtn>>,
) {
    for btn in &mut btns {
        if *btn.0 == Interaction::Pressed {
            next_state.set(GameState::StyleSelect);
        }
    }
    for mut btn in &mut btns {
        match *btn.0 {
            Interaction::None => {
                *btn.1 = BackgroundColor::from(Color::NONE);
            }
            Interaction::Hovered => {
                *btn.1 = BackgroundColor::from(Color::rgba(1.0, 1.0, 1.0, 0.1));
            }
            Interaction::Pressed => {
                *btn.1 = BackgroundColor::from(Color::rgba(1.0, 1.0, 1.0, 0.2));
            }
        }
    }
}
