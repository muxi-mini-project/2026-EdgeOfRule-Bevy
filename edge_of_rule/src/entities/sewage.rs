use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Sewage;

pub fn spawn_sewage(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/sewage.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Sewage,
    ));
}
