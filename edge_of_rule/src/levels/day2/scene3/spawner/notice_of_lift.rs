use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, lift::Lift, player::Player, press_e::spawn_press_e},
    levels::day2::scene3::actions::LiftState,
};

#[derive(Component)]
pub struct NoticeOfLift;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices: Query<&NoticeOfLift>,
    players: Query<&Transform, With<Player>>,
    notice_boards: Query<&Transform, With<Lift>>,
    lift_state: Res<LiftState>,
) {
    if *lift_state != LiftState::Fixed {
        return;
    }
    if notices.iter().len() != 0 {
        return;
    }

    for player in &players {
        for notice_board in &notice_boards {
            if (player.translation.x - notice_board.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(-48.0, 160.0, 1.0),
                    &asset_server,
                    NoticeOfLift,
                );
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(28.0, 160.0, 1.0),
                    &asset_server,
                    "进入",
                    NoticeOfLift,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfLift>>,
    players: Query<&Transform, With<Player>>,
    notice_boards: Query<&Transform, With<Lift>>,
    lift_state: Res<LiftState>,
) {
    for player in &players {
        if *lift_state == LiftState::Broken {
            break;
        }
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

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfLift>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}
