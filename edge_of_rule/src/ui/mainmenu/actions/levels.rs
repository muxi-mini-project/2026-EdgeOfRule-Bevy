use bevy::prelude::*;

use crate::{core::state::GameState::{self, Day1, Day2, Day3, Day4, Day5, Day6, Day7}, ui::mainmenu::spawner::levels::{
    LevelDay1, LevelDay2, LevelDay3, LevelDay4, LevelDay5, LevelDay6, LevelDay7,
}};

pub fn on_click_day1 (
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction,&LevelDay1)>,
) {
    for (reaction,_) in &mut btns {
        if *reaction == Interaction::Pressed {
            next_state.set(Day1);
        }
    }
}

pub fn on_click_day2 (
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction,&LevelDay2)>,
) {
    for (reaction,_) in &mut btns {
        if *reaction == Interaction::Pressed {
            next_state.set(Day2);
        }
    }
}

pub fn on_click_day3 (
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction,&LevelDay3)>,
) {
    for (reaction,_) in &mut btns {
        if *reaction == Interaction::Pressed {
            next_state.set(Day3);
        }
    }
}

pub fn on_click_day4 (
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction,&LevelDay4)>,
) {
    for (reaction,_) in &mut btns {
        if *reaction == Interaction::Pressed {
            next_state.set(Day4);
        }
    }
}

pub fn on_click_day5 (
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction,&LevelDay5)>,
) {
    for (reaction,_) in &mut btns {
        if *reaction == Interaction::Pressed {
            next_state.set(Day5);
        }
    }
}

pub fn on_click_day6 (
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction,&LevelDay6)>,
) {
    for (reaction,_) in &mut btns {
        if *reaction == Interaction::Pressed {
            next_state.set(Day6);
        }
    }
}

pub fn on_click_day7 (
    mut next_state: ResMut<NextState<GameState>>,
    mut btns: Query<(&Interaction,&LevelDay7)>,
) {
    for (reaction,_) in &mut btns {
        if *reaction == Interaction::Pressed {
            next_state.set(Day7);
        }
    }
}