mod gameplay;
mod ui;

use crate::gameplay::GamePlayPlugin;
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
        .add_plugins(UiPlugin)
        .add_plugins(GamePlayPlugin)
        .run();
}
