use bevy::prelude::*;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    MainMenu,
    Options,
    StyleSelect,
    LevelSelect,
    RulesView,

    Day1Scene1,
    Day1Scene2,
    Day1Scene3,
    Day1Scene4,

    Day2Scene1,
    Day2Scene2,
    Day2Scene3,
    #[default]
    Day2Scene4,
    Day2Scene5,

    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    InGameOption,
}
