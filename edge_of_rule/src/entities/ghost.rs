use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Ghost;

#[derive(Component, Debug, Clone)]
pub struct GhostAssets {
    pub idle_or_left: Handle<Image>,
    pub right: Handle<Image>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct GhostMotion {
    pub last_pos: Vec2,
}

pub fn spawn_ghost(commands: &mut Commands, transform: Transform, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/ghost/0.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Ghost,
        GhostAssets {
            idle_or_left: asset_server.load("images/animations/ghost/0.png"),
            right: asset_server.load("images/animations/ghost/1.png"),
        },
        GhostMotion {
            last_pos: transform.translation.truncate(),
        },
    ));
}
