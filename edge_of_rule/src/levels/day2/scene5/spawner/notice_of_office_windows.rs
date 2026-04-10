use bevy::prelude::*;

use crate::entities::{
    arrow::spawn_arrow, player::Player, press_e::spawn_press_e, windows::Windows,
};

#[derive(Component)]
pub struct NoticeOfOfficeWindows;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices: Query<&NoticeOfOfficeWindows>,
    players: Query<&Transform, With<Player>>,
    windows: Query<&Transform, With<Windows>>,
) {
    if notices.iter().len() != 0 {
        return;
    }

    let Ok(win_tf) = windows.get_single() else {
        return;
    };

    for player_tf in &players {
        if (player_tf.translation.x - win_tf.translation.x).abs() < 60.0 {
            let arrow_pos =
                Transform::from_xyz(win_tf.translation.x, win_tf.translation.y + 60.0, 1.0);
            let press_e_pos = Transform::from_xyz(
                win_tf.translation.x + 76.0,
                win_tf.translation.y + 60.0,
                1.0,
            );
            spawn_arrow(
                &mut commands,
                arrow_pos,
                &asset_server,
                NoticeOfOfficeWindows,
            );
            spawn_press_e(
                &mut commands,
                press_e_pos,
                &asset_server,
                "翻越",
                NoticeOfOfficeWindows,
            );
            return;
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfOfficeWindows>>,
    players: Query<&Transform, With<Player>>,
    windows: Query<&Transform, With<Windows>>,
) {
    let Ok(win_tf) = windows.get_single() else {
        for e in &notices {
            commands.entity(e).despawn_recursive();
        }
        return;
    };

    for player_tf in &players {
        if (player_tf.translation.x - win_tf.translation.x).abs() < 60.0 {
            return;
        }
    }

    for e in &notices {
        commands.entity(e).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfOfficeWindows>>) {
    for e in &notices {
        commands.entity(e).despawn_recursive();
    }
}
