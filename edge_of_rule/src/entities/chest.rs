use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Chest;

pub fn spawn_chest(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/chest.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Chest,
    ));
}
