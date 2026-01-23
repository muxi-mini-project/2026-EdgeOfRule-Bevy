use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Trapdoor;

pub fn spawn_trapdoor(
    commands: &mut Commands,
    transform: Transform,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/trapdoor.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Trapdoor,
    ));
}
