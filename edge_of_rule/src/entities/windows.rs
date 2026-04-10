use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Windows;

pub fn spawn_windows(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/windows.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Windows,
    ));
}
