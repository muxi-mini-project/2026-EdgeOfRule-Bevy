use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Table;

pub fn spawn_table(commands: &mut Commands, transform: Transform, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/table.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Table,
    ));
}
