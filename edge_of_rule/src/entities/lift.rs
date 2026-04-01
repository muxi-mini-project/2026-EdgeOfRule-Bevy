use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Lift;

pub fn spawn_lift(commands: &mut Commands, transform: Transform, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/lift/broken.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Lift,
    ));
}
