use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Circuit;

pub fn spawn_curcuit(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/circuit.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Circuit,
    ));
}
