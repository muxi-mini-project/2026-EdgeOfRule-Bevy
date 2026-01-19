mod animation;
mod assets;
mod constants;
mod core;
mod entities;
mod levels;
mod physics;
mod ui;
mod utils;

use crate::assets::AssetsPlugin;
use crate::core::CorePlugin;
use crate::entities::EntitiesPlugin;
use crate::ui::UiPlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "校园疾跑：规则边缘".to_string(),
                        mode: WindowMode::Fullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(CorePlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(EntitiesPlugin)
        .add_plugins(UiPlugin)
        .run();
}
