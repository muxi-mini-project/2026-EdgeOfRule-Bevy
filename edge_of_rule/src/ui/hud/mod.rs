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
            .add_systems(Startup, actions::esc_ingame_option::spawn)
            .add_systems(
                Update,
                    (
                        actions::esc_ingame_option::on_key_esc,
                        crate::ui::mainmenu::actions::exit_game_btn::on_click,
                    ),
            );
    }
}


