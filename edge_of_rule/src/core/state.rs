use bevy::prelude::*;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Options,
    StyleSelect,
    LevelSelect,
    RulesView,
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
}
