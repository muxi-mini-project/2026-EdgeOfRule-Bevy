use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Log;

pub fn spawn_log(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/log.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Log,
    ));
}
