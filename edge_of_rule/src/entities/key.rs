use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Key;

pub fn spawn_key(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/key.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Key,
    ));
}
