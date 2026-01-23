use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Mirror;

pub fn spawn_mirror(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/mirror.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Mirror,
    ));
}
