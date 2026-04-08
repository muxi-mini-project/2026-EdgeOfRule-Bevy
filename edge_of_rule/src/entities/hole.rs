use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Hole;

pub fn spawn_hole(commands: &mut Commands, transform: Transform, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/hole.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Hole,
    ));
}
