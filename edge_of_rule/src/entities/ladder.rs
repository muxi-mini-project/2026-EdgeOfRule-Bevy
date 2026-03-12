use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Ladder;

pub fn spawn_ladder(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/ladder.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Ladder,
    ));
}
