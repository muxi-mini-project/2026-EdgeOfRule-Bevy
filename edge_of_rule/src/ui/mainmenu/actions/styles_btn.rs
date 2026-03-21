use bevy::prelude::*;

use crate::{core::state::GameState, ui::mainmenu::spawner::styles_keys::{EasyBtn, HardBtn, MediumBtn, StyleBtnState, ButtonSelectionState}};



pub fn on_click_easy(
    mut btns: Query<(&Interaction,&mut BackgroundColor, &mut StyleBtnState), With<EasyBtn>>,
    mut selection_state: ResMut<ButtonSelectionState>,
) {
    for (reaction,mut color, mut state) in &mut btns {
        if selection_state.is_selected && !state.pressed {
            continue;
        }

        if state.pressed {
            *color = BackgroundColor::from(Color::RED);
            continue;
        }

        match *reaction {
            Interaction::Pressed => {
                    if !selection_state.is_selected {
                    *color = BackgroundColor::from(Color::RED);
                    state.pressed = true;
                    selection_state.is_selected = true;
                    selection_state.selected_btn = Some("easy".to_string());
                }
            }
            Interaction::Hovered => {
                if !selection_state.is_selected {
                    *color = BackgroundColor::from(Color::rgb(31.0 / 255.0, 56.0 / 255.0, 72.0 / 255.0));
                }
            }
            Interaction::None => {
                *color = BackgroundColor::from(Color::WHITE);
            }
        }
    }
}

pub fn on_click_mid(
    mut btns: Query<(&Interaction,&mut BackgroundColor, &mut StyleBtnState), With<MediumBtn>>,
    mut selection_state: ResMut<ButtonSelectionState>,
) {
    for (reaction,mut color, mut state) in &mut btns {
        if selection_state.is_selected && !state.pressed {
            continue;
        }
        if state.pressed {
            *color = BackgroundColor::from(Color::RED);
            continue;
        }

        match *reaction {
            Interaction::Pressed => {
                    if !selection_state.is_selected {
                    *color = BackgroundColor::from(Color::rgb(16.0 / 255.0, 30.0 / 255.0, 39.0 / 255.0));
                    state.pressed = true;
                    selection_state.is_selected = true;
                    selection_state.selected_btn = Some("medium".to_string());
                }
            }
            Interaction::Hovered => {
                if !selection_state.is_selected {
                    *color = BackgroundColor::from(Color::rgb(31.0 / 255.0, 56.0 / 255.0, 72.0 / 255.0));
                }
            }
            Interaction::None => {
                *color = BackgroundColor::from(Color::WHITE);
            }
        }
    }
}

pub fn on_click_hard(
    mut btns: Query<(&Interaction,&mut BackgroundColor, &mut StyleBtnState), With<HardBtn>>,
    mut selection_state: ResMut<ButtonSelectionState>,
) {
    for (reaction,mut color, mut state) in &mut btns {
        if selection_state.is_selected && !state.pressed {
            continue;
        }
        if state.pressed {
            *color = BackgroundColor::from(Color::RED);
            continue;
        }

        match *reaction {
            Interaction::Pressed => {
                if !selection_state.is_selected {
                    *color = BackgroundColor::from(Color::rgb(16.0 / 255.0, 30.0 / 255.0, 39.0 / 255.0));
                    state.pressed = true;
                    selection_state.is_selected = true;
                    selection_state.selected_btn = Some("hard".to_string());
                }
            }
            Interaction::Hovered => {
                if !selection_state.is_selected {
                    *color = BackgroundColor::from(Color::rgb(31.0 / 255.0, 56.0 / 255.0, 72.0 / 255.0));
                }
            }
            Interaction::None => {
                *color = BackgroundColor::from(Color::WHITE);
            }
        }
    }
}