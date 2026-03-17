use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Controller;

pub fn spawn_controller(
    commands: &mut Commands,
    transform: Transform,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/controller.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Controller,
    ));
}
