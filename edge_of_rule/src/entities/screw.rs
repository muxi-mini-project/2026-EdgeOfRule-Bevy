use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Screw;

pub fn spawn_screw(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/screw.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Screw,
    ));
}
