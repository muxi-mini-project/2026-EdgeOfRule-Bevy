use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct SmallNote;

pub fn spawn_small_note(
    commands: &mut Commands,
    transform: Transform,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/small_note.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        SmallNote,
    ));
}
