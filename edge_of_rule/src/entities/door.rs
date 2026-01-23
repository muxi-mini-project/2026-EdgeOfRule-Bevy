use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Door;

pub fn spawn_door(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/door.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Door,
    ));
}
