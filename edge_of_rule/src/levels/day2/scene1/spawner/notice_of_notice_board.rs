use bevy::prelude::*;

use crate::entities::{
    arrow::spawn_arrow, notice_board::NoticeBoard, player::Player, press_e::spawn_press_e,
};

#[derive(Component)]
pub struct NoticeOfNoticeBoard;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices: Query<&NoticeOfNoticeBoard>,
    players: Query<&Transform, With<Player>>,
    notice_boards: Query<&Transform, With<NoticeBoard>>,
) {
    if notices.iter().len() != 0 {
        return;
    }

    for player in &players {
        for notice_board in &notice_boards {
            if (player.translation.x - notice_board.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(-268.0, 82.0, 1.0),
                    &asset_server,
                    NoticeOfNoticeBoard,
                );
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(-208.0, 82.0, 1.0),
                    &asset_server,
                    "查看",
                    NoticeOfNoticeBoard,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfNoticeBoard>>,
    players: Query<&Transform, With<Player>>,
    notice_boards: Query<&Transform, With<NoticeBoard>>,
) {
    for player in &players {
        for notice_board in &notice_boards {
            if (player.translation.x - notice_board.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfNoticeBoard>>) {
    for notice in &notices {
        commands.entity(notice).despawn();
    }
}
