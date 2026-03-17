use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Cover;

pub fn spawn_cover(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/cover.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Cover,
    ));
}
