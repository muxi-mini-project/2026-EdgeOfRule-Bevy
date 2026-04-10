use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, log::Log, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct NoticeOfOfficeLog;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices: Query<&NoticeOfOfficeLog>,
    players: Query<&Transform, With<Player>>,
    logs: Query<&Transform, With<Log>>,
) {
    if notices.iter().len() != 0 {
        return;
    }

    let Ok(log_tf) = logs.get_single() else {
        return;
    };

    for player_tf in &players {
        if (player_tf.translation.x - log_tf.translation.x).abs() < 60.0 {
            let arrow_pos =
                Transform::from_xyz(log_tf.translation.x, log_tf.translation.y + 60.0, 1.0);
            let press_e_pos = Transform::from_xyz(
                log_tf.translation.x + 76.0,
                log_tf.translation.y + 60.0,
                1.0,
            );
            spawn_arrow(&mut commands, arrow_pos, &asset_server, NoticeOfOfficeLog);
            spawn_press_e(
                &mut commands,
                press_e_pos,
                &asset_server,
                "阅读",
                NoticeOfOfficeLog,
            );
            return;
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfOfficeLog>>,
    players: Query<&Transform, With<Player>>,
    logs: Query<&Transform, With<Log>>,
) {
    let Ok(log_tf) = logs.get_single() else {
        // If log doesn't exist, ensure stale notice is removed.
        for e in &notices {
            commands.entity(e).despawn_recursive();
        }
        return;
    };

    for player_tf in &players {
        if (player_tf.translation.x - log_tf.translation.x).abs() < 60.0 {
            return;
        }
    }

    for e in &notices {
        commands.entity(e).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfOfficeLog>>) {
    for e in &notices {
        commands.entity(e).despawn_recursive();
    }
}
