use bevy::prelude::*;

use crate::{core::state::GameState, ui::mainmenu::spawner::styles_btn::{EasyBtn, HardBtn, MediumBtn}};

pub fn on_click(
    // mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction,&mut BackgroundColor), Or<(With<EasyBtn>,With<MediumBtn>,With<HardBtn>)>>,
) {
    for (reaction,mut color) in &mut btns {
        match *reaction {
            Interaction::Pressed => {
                // if let Ok(_) = btns.get_component::<EasyBtn>(reaction.entity()) {
                //     next_state.set(GameState::StyleEasy);
                // } else if let Ok(_) = btns.get_component::<MediumBtn>(reaction.entity()) {
                //     next_state.set(GameState::StyleMedium);
                // } else if let Ok(_) = btns.get_component::<HardBtn>(reaction.entity()) {
                //     next_state.set(GameState::StyleHard);
                // }
                *color = BackgroundColor::from(Color::rgb(16.0 / 255.0, 30.0 / 255.0, 39.0 / 255.0));
            }
            Interaction::Hovered => {
                *color = BackgroundColor::from(Color::rgb(31.0 / 255.0, 56.0 / 255.0, 72.0 / 255.0));
            }
            Interaction::None => {
                *color = BackgroundColor::from(Color::rgb(24.0 / 255.0, 43.0 / 255.0, 56.0 / 255.0));
            }
        }
    }
}