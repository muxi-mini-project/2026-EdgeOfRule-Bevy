use bevy::prelude::*;

use crate::entities::notice_board::{NoticeBoard, spawn_notice_board};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_notice_board(&mut commands, Transform::from_xyz(-268.0, 8.0, -4.0), asset);
}

pub fn despawn(mut commands: Commands, doors: Query<Entity, With<NoticeBoard>>) {
    for door in &doors {
        commands.entity(door).despawn();
    }
}
