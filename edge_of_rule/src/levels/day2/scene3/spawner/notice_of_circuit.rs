use bevy::prelude::*;

use crate::entities::{
    arrow::spawn_arrow, circuit::Circuit, player::Player, press_e::spawn_press_e,
};

#[derive(Component)]
pub struct NoticeOfCircuit;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices: Query<&NoticeOfCircuit>,
    players: Query<&Transform, With<Player>>,
    notice_boards: Query<&Transform, With<Circuit>>,
) {
    if notices.iter().len() != 0 {
        return;
    }

    for player in &players {
        for notice_board in &notice_boards {
            if (player.translation.x - notice_board.translation.x).abs() < 30.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(48.0, 20.0, 1.0),
                    &asset_server,
                    NoticeOfCircuit,
                );
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(108.0, 20.0, 1.0),
                    &asset_server,
                    "查看",
                    NoticeOfCircuit,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfCircuit>>,
    players: Query<&Transform, With<Player>>,
    notice_boards: Query<&Transform, With<Circuit>>,
) {
    for player in &players {
        for notice_board in &notice_boards {
            if (player.translation.x - notice_board.translation.x).abs() < 30.0 {
                return;
            }
        }
    }

    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfCircuit>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}
