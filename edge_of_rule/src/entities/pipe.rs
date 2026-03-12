use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Pipe;

pub fn spawn_pipe(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/pipe.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Pipe,
    ));
}
