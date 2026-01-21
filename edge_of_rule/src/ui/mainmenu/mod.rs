pub mod actions;
pub mod spawner;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (
                spawner::titled_background::spawn,
                spawner::options_btn::spawn,
                spawner::style_select_btn::spawn,
                spawner::game_start_btn::spawn,
                spawner::rules_btn::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (
                spawner::titled_background::despawn,
                spawner::options_btn::despawn,
                spawner::style_select_btn::despawn,
                spawner::game_start_btn::despawn,
                spawner::rules_btn::despawn,
            ),
        )
        .add_systems(
            OnEnter(GameState::Options),
            (
                spawner::background::spawn,
                spawner::options_btn::spawn,
                spawner::backward_btn::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::Options),
            (
                spawner::background::despawn,
                spawner::options_btn::despawn,
                spawner::backward_btn::despawn,
            ),
        )
        .add_systems(
            OnEnter(GameState::StyleSelect),
            (
                spawner::background::spawn,
                spawner::options_btn::spawn,
                spawner::backward_btn::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::StyleSelect),
            (
                spawner::background::despawn,
                spawner::options_btn::despawn,
                spawner::backward_btn::despawn,
            ),
        )
        .add_systems(
            OnEnter(GameState::LevelSelect),
            (
                spawner::background::spawn,
                spawner::options_btn::spawn,
                spawner::backward_btn::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::LevelSelect),
            (
                spawner::background::despawn,
                spawner::options_btn::despawn,
                spawner::backward_btn::despawn,
            ),
        )
        .add_systems(
            OnEnter(GameState::RulesView),
            (
                spawner::background::spawn,
                spawner::options_btn::spawn,
                spawner::backward_btn::spawn,
                spawner::rules::spawn,
            ),
        )
        .add_systems(
            OnExit(GameState::RulesView),
            (
                spawner::background::despawn,
                spawner::options_btn::despawn,
                spawner::backward_btn::despawn,
                spawner::rules::despawn,
            ),
        )
        .add_systems(
            Update,
            (
                actions::options_btn::on_click,
                actions::style_select_btn::on_click,
                actions::game_start_btn::on_click,
                actions::rules_btn::on_click,
                actions::backward_btn::on_click,
            ),
        );
    }
}
