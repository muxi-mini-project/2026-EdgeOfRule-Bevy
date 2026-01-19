mod gameplay;
mod ui;

use crate::ui::UiPlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "校园疾跑：规则边缘".to_string(),
                mode: WindowMode::Fullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UiPlugin)
        .run();
}
