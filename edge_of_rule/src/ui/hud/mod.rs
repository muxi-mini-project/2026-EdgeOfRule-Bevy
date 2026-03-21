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

            .add_systems(
                Update,
                    (
                        actions::esc_ingame_option::on_key_esc,
                        actions::esc_ingame_option::on_click_exit,
                        actions::esc_ingame_option::on_click_menu,
                        actions::back_to_mainmenu_btn::on_click,
                    ),
            );
    }
}


