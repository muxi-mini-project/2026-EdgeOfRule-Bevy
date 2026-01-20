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

use crate::assets::AssetsPlugin;
use crate::control::ControlPlugin;
use crate::core::CorePlugin;
use crate::entities::EntitiesPlugin;
use crate::ui::UiPlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::*;

fn main() {
    let default_plugin = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "校园疾跑：规则边缘".to_string(),
                mode: WindowMode::Fullscreen,
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest());

    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(default_plugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(CorePlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(EntitiesPlugin)
        .add_plugins(ControlPlugin)
        .add_plugins(UiPlugin)
        .run();
}
