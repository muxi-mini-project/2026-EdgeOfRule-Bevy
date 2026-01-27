use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Arrow;

pub fn spawn_arrow(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
    component: impl Component,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/HUD/arrow/0.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Arrow,
        component,
    ));
}
