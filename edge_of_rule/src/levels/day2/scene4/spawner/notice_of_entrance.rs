use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct NoticeOfEntrance;

#[derive(Component)]
pub struct NoticeOfExit;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices1: Query<&NoticeOfEntrance>,
    notices2: Query<&NoticeOfExit>,
    players: Query<&Transform, With<Player>>,
) {
    if notices1.iter().len() != 0 {
        return;
    }
    if notices2.iter().len() != 0 {
        return;
    }

    for player in &players {
        if (player.translation.x - (-450.0)).abs() < 60.0
            && (player.translation.y - (-250.0)).abs() < 60.0
        {
            spawn_arrow(
                &mut commands,
                Transform::from_xyz(-450.0, -200.0, 25.0),
                &asset_server,
                NoticeOfEntrance,
            );
            spawn_press_e(
                &mut commands,
                Transform::from_xyz(-390.0, -200.0, 25.0),
                &asset_server,
                "返回",
                NoticeOfEntrance,
            );
        }
        if (player.translation.x - (450.0)).abs() < 60.0
            && (player.translation.y - (250.0)).abs() < 60.0
        {
            spawn_arrow(
                &mut commands,
                Transform::from_xyz(450.0, 300.0, 25.0),
                &asset_server,
                NoticeOfExit,
            );
            spawn_press_e(
                &mut commands,
                Transform::from_xyz(510.0, 300.0, 25.0),
                &asset_server,
                "进入",
                NoticeOfExit,
            );
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices1: Query<Entity, With<NoticeOfEntrance>>,
    notices2: Query<Entity, With<NoticeOfExit>>,
    players: Query<&Transform, With<Player>>,
) {
    for player in &players {
        if (player.translation.x - (-450.0)).abs() < 60.0
            && (player.translation.y - (-250.0)).abs() < 60.0
        {
            return;
        }
        if (player.translation.x - (450.0)).abs() < 60.0
            && (player.translation.y - (250.0)).abs() < 60.0
        {
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

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfEntrance>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}

pub fn despawn_all_exit(mut commands: Commands, notices: Query<Entity, With<NoticeOfExit>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}
