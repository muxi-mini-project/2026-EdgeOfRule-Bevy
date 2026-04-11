use bevy::prelude::*;

use crate::{
    core::state::GameState::{self, Day1Scene1, Day2Scene1, Day3, Day4, Day5, Day6, Day7},
    ui::mainmenu::spawner::{
            levels::{
            LevelDay1, LevelDay2, LevelDay3, LevelDay4, LevelDay5, LevelDay6, LevelDay7,
        },
        lock_levels::{
            LevelOneLock, LevelTwoLock, LevelThreeLock, LevelFourLock, LevelFiveLock, LevelSixLock, LevelSevenLock,
        },
        levels_warn_popups::{
            WarningPopup,
            WarningPopupState,
        },
    },
};



pub fn on_click_day1(
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction, &LevelDay1)>,
    day_one_lock: ResMut<LevelOneLock>,
) {
    if day_one_lock.locked {
        return;
    }
    for (reaction, _) in &mut btns {
        if *reaction == Interaction::Pressed {
            next_state.set(Day1Scene1);
        }
    }
}

pub fn on_click_day2(
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction, &LevelDay2)>,
    day_two_lock: ResMut<LevelTwoLock>,
    mut warning_state: ResMut<WarningPopupState>,
) {
    
    for (reaction, _) in &mut btns {
        if *reaction == Interaction::Pressed {
            if day_two_lock.locked {
                warning_state.structed = true;
                return;
            }
            next_state.set(Day2Scene1);
        }
    }
}

pub fn on_click_day3(
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction, &LevelDay3)>,
    day_three_lock: ResMut<LevelThreeLock>,
    mut warning_state: ResMut<WarningPopupState>,
) {
    
    for (reaction, _) in &mut btns {
        if *reaction == Interaction::Pressed {
            if day_three_lock.locked {
                warning_state.structed = true;
                return;
            }
            next_state.set(Day3);
        }
    }
}

pub fn on_click_day4(
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction, &LevelDay4)>,
    day_four_lock: ResMut<LevelFourLock>,
    mut warning_state: ResMut<WarningPopupState>,
) {
    
    for (reaction, _) in &mut btns {
        if *reaction == Interaction::Pressed {
            if day_four_lock.locked {
                warning_state.structed = true;

                return;
            }
            next_state.set(Day4);
        }
    }
}

pub fn on_click_day5(
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction, &LevelDay5)>,
    day_five_lock: ResMut<LevelFiveLock>,
    mut warning_state: ResMut<WarningPopupState>,
) {
    
    for (reaction, _) in &mut btns {
        if *reaction == Interaction::Pressed {
            if day_five_lock.locked {
                warning_state.structed = true;
                return;
            }
            next_state.set(Day5);
        }
    }
}

pub fn on_click_day6(
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction, &LevelDay6)>,
    day_six_lock: ResMut<LevelSixLock>,
    mut warning_state: ResMut<WarningPopupState>,
) {
    
    for (reaction, _) in &mut btns {
        if *reaction == Interaction::Pressed {
            if day_six_lock.locked {
                warning_state.structed = true;

                return;
            }
            next_state.set(Day6);
        }
    }
}

pub fn on_click_day7(
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction, &LevelDay7)>,
    day_seven_lock: ResMut<LevelSevenLock>,
    mut warning_state: ResMut<WarningPopupState>,
) {
    
    for (reaction, _) in &mut btns {
        if *reaction == Interaction::Pressed {
            if day_seven_lock.locked {
                warning_state.structed = true;
                return;
            }
            next_state.set(Day7);
        }
    }
}

