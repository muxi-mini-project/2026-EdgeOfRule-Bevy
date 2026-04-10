use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Desk;

pub fn spawn_desk(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
    variant: usize,
) {
    let texture_path = format!("images/animations/desk/{}.png", variant);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(texture_path),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Desk,
    ));
}
