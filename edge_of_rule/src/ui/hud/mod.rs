pub mod actions;
pub mod spawner;

use crate::ui::mainmenu;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(actions::esc_ingame_option::OptionSpawnState { is_visible: bool::default() })
            .add_systems(Startup, spawner::ingame_option_area::spawn_ingame_option_area)
            .add_systems(Startup, spawner::ingame_option_area::spawn_in_game_option_title)
            .add_systems(Startup, spawner::ingame_option_area::spawn_exit_game_btn)
            .add_systems(Startup, spawner::ingame_option_area::spawn_under_tip)
            .add_systems(Startup, spawner::ingame_option_area::spawn_back_to_mainmenu_btn)
            .add_systems(Startup, spawner::ingame_option_area::spawn_keys_tip)
            .add_systems(Startup, spawner::ingame_option_area::spawn_keys_words)


            .add_systems(OnEnter(GameState::Day1Scene1), spawner::blood_bar::spawn_blood_bar)
            .add_systems(OnEnter(GameState::Day1Scene2), spawner::blood_bar::spawn_blood_bar)
            .add_systems(OnEnter(GameState::Day1Scene3), spawner::blood_bar::spawn_blood_bar)
            .add_systems(OnEnter(GameState::Day1Scene4), spawner::blood_bar::spawn_blood_bar)

            .add_systems(OnEnter(GameState::Day2Scene1), spawner::blood_bar::spawn_blood_bar)
            .add_systems(OnEnter(GameState::Day2Scene2), spawner::blood_bar::spawn_blood_bar)
            .add_systems(OnEnter(GameState::Day2Scene3), spawner::blood_bar::spawn_blood_bar)
            .add_systems(OnEnter(GameState::Day2Scene4), spawner::blood_bar::spawn_blood_bar)

            .add_systems(OnExit(GameState::Day1Scene1), spawner::blood_bar::despawn_blood_bar)
            .add_systems(OnExit(GameState::Day1Scene2), spawner::blood_bar::despawn_blood_bar)
            .add_systems(OnExit(GameState::Day1Scene3), spawner::blood_bar::despawn_blood_bar)
            .add_systems(OnExit(GameState::Day1Scene4), spawner::blood_bar::despawn_blood_bar)

            .add_systems(OnExit(GameState::Day2Scene1), spawner::blood_bar::despawn_blood_bar)
            .add_systems(OnExit(GameState::Day2Scene2), spawner::blood_bar::despawn_blood_bar)
            .add_systems(OnExit(GameState::Day2Scene3), spawner::blood_bar::despawn_blood_bar)
            .add_systems(OnExit(GameState::Day2Scene4), spawner::blood_bar::despawn_blood_bar)

            .add_systems(
                Update,
                    (
                        actions::blood_bar::update_blood_bar,
                        actions::esc_ingame_option::on_key_esc,
                        actions::esc_ingame_option::on_click_exit,
                        actions::esc_ingame_option::on_click_menu,
                        actions::esc_ingame_option::on_click_keys_tip,
                        actions::back_to_mainmenu_btn::on_click,
                    ),
            );
    }
}


