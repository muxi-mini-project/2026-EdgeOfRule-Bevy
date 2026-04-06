use bevy::prelude::*;

use crate::{ui::mainmenu::spawner::styles_keys::{EasyBtn, HardBtn, MediumBtn, HardBtnState, EasyBtnState, MidBtnState}};



pub fn on_click_easy(
    mut btns: Query<(&Interaction,&mut BackgroundColor), With<EasyBtn>>,
    mut easy_state: ResMut<EasyBtnState>,
    mut mid_state: ResMut<MidBtnState>,
    mut hard_state: ResMut<HardBtnState>,
) {
    for (reaction,mut color) in &mut btns {
        
        match *reaction {
            Interaction::Pressed => {
                    *color = BackgroundColor::from(Color::RED);
                       
                    mid_state.pressed = false;
                    hard_state.pressed = false;
                    easy_state.pressed = true; 
            }
            Interaction::Hovered => {
                    *color = BackgroundColor::from(Color::rgb(31.0 / 255.0, 56.0 / 255.0, 72.0 / 255.0));
                }
            Interaction::None => {
                *color = BackgroundColor::from(Color::WHITE);
            }
        }
        if easy_state.pressed {
            *color = BackgroundColor::from(Color::RED);
            continue;
        }
    }
}

pub fn on_click_mid(
    mut btns: Query<(&Interaction,&mut BackgroundColor), With<MediumBtn>>,
    mut easy_state: ResMut<EasyBtnState>,
    mut mid_state: ResMut<MidBtnState>,
    mut hard_state: ResMut<HardBtnState>,
) {
    for (reaction,mut color) in &mut btns {

        match *reaction {
            Interaction::Pressed => {
                    *color = BackgroundColor::from(Color::rgb(16.0 / 255.0, 30.0 / 255.0, 39.0 / 255.0));
                    easy_state.pressed = false;
                    hard_state.pressed = false;
                    mid_state.pressed = true;
                }
            
            Interaction::Hovered => {
                    *color = BackgroundColor::from(Color::rgb(31.0 / 255.0, 56.0 / 255.0, 72.0 / 255.0));
            }
            Interaction::None => {
                *color = BackgroundColor::from(Color::WHITE);
            }
        }

        if  mid_state.pressed {
            *color = BackgroundColor::from(Color::RED);
            continue;
        }
    }
}

pub fn on_click_hard(
    mut btns: Query<(&Interaction,&mut BackgroundColor), With<HardBtn>>,
    mut easy_state: ResMut<EasyBtnState>,
    mut mid_state: ResMut<MidBtnState>,
    mut hard_state: ResMut<HardBtnState>,
) {
    for (reaction,mut color) in &mut btns {

        match *reaction {
            Interaction::Pressed => {
                    *color = BackgroundColor::from(Color::rgb(16.0 / 255.0, 30.0 / 255.0, 39.0 / 255.0));
                    easy_state.pressed = false;
                    mid_state.pressed = false;
                    hard_state.pressed = true;
            }
            Interaction::Hovered => {
                    *color = BackgroundColor::from(Color::rgb(31.0 / 255.0, 56.0 / 255.0, 72.0 / 255.0));
            }
            Interaction::None => {
                *color = BackgroundColor::from(Color::WHITE);
            }
        }

        if hard_state.pressed {
            *color = BackgroundColor::from(Color::RED);
            continue;
        }
    }
}