mod mainmenu;
mod hud;

use crate::{assets::ui_image::load_ui_image_assets, ui::mainmenu::MainMenuPlugin,
    //  ui::hud::HudPlugin
    };
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Startup, load_ui_image_assets)
            .add_plugins(MainMenuPlugin);
            // .add_plugins(HudPlugin);
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
