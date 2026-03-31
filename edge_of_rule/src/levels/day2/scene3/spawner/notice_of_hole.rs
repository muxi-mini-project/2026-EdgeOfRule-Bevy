use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, hole::Hole, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct NoticeOfHole;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices: Query<&NoticeOfHole>,
    players: Query<&Transform, With<Player>>,
    notice_boards: Query<&Transform, With<Hole>>,
) {
    if notices.iter().len() != 0 {
        return;
    }

    for player in &players {
        for notice_board in &notice_boards {
            if (player.translation.x - notice_board.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(104.0, 20.0, 1.0),
                    &asset_server,
                    NoticeOfHole,
                );
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(164.0, 20.0, 1.0),
                    &asset_server,
                    "进入",
                    NoticeOfHole,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfHole>>,
    players: Query<&Transform, With<Player>>,
    notice_boards: Query<&Transform, With<Hole>>,
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

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfHole>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}
