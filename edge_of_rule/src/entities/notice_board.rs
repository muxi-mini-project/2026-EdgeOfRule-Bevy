use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct NoticeBoard;

pub fn spawn_notice_board(
    commands: &mut Commands,
    transform: Transform,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/animations/notice_board.png"),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        NoticeBoard,
    ));
}
