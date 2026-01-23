use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct BrokenFloor;

pub fn spawn_broken_floor(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/broken_floor.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        BrokenFloor,
    ));
}
