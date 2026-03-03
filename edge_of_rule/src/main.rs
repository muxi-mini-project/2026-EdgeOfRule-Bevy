mod animation;
mod assets;
mod constants;
mod control;
mod core;
mod entities;
mod levels;
mod physics;
mod ui;
mod utils;

use crate::animation::AnimationPlugin;
use crate::assets::AssetsPlugin;
use crate::control::ControlPlugin;
use crate::core::CorePlugin;
use crate::levels::LevelsPlugin;
use crate::ui::UiPlugin;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_rapier2d::prelude::*;

fn main() {
    let default_plugin = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "校园疾跑：规则边缘".to_string(),
                visible: false,
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest());

    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(default_plugin)
        .add_systems(Startup, start_fullscreen)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(CorePlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(ControlPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(LevelsPlugin)
        .add_plugins(UiPlugin)
        .run();
}

fn start_fullscreen(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.mode = WindowMode::BorderlessFullscreen;
    window.visible = true;
}
