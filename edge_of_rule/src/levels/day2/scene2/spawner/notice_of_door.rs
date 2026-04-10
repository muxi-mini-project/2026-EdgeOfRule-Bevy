use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, player::Player, press_e::spawn_press_e};

use super::door::{LEFT_DOOR_POS, RIGHT_DOOR_POS};

#[derive(Component)]
pub struct NoticeOfLeftDoor;

#[derive(Component)]
pub struct NoticeOfRightDoor;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices1: Query<&NoticeOfLeftDoor>,
    notices2: Query<&NoticeOfRightDoor>,
    players: Query<&Transform, With<Player>>,
) {
    if notices1.iter().len() != 0 {
        return;
    }
    if notices2.iter().len() != 0 {
        return;
    }

    for player in &players {
        if (player.translation.x - LEFT_DOOR_POS.x).abs() < 60.0 {
            let arrow_pos = Transform::from_xyz(LEFT_DOOR_POS.x, 120.0, 1.0);
            let press_e_pos = Transform::from_xyz(LEFT_DOOR_POS.x + 76.0, 120.0, 1.0);
            spawn_arrow(&mut commands, arrow_pos, &asset_server, NoticeOfLeftDoor);
            spawn_press_e(
                &mut commands,
                press_e_pos,
                &asset_server,
                "出门",
                NoticeOfLeftDoor,
            );
            return;
        }

        if (player.translation.x - RIGHT_DOOR_POS.x).abs() < 60.0 {
            let arrow_pos = Transform::from_xyz(RIGHT_DOOR_POS.x, 120.0, 1.0);
            let press_e_pos = Transform::from_xyz(RIGHT_DOOR_POS.x - 76.0, 120.0, 1.0);
            spawn_arrow(&mut commands, arrow_pos, &asset_server, NoticeOfRightDoor);
            spawn_press_e(
                &mut commands,
                press_e_pos,
                &asset_server,
                "出门",
                NoticeOfRightDoor,
            );
            return;
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices1: Query<Entity, With<NoticeOfLeftDoor>>,
    notices2: Query<Entity, With<NoticeOfRightDoor>>,
    players: Query<&Transform, With<Player>>,
) {
    for player in &players {
        if (player.translation.x - LEFT_DOOR_POS.x).abs() < 60.0 {
            return;
        }
        if (player.translation.x - RIGHT_DOOR_POS.x).abs() < 60.0 {
            return;
        }
    }

    for notice in &notices1 {
        commands.entity(notice).despawn_recursive();
    }
    for notice in &notices2 {
        commands.entity(notice).despawn_recursive();
    }
}

pub fn despawn_all_left(mut commands: Commands, notices: Query<Entity, With<NoticeOfLeftDoor>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}

pub fn despawn_all_right(mut commands: Commands, notices: Query<Entity, With<NoticeOfRightDoor>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}
