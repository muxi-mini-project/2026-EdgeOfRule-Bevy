use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct NoticeOfEntrance;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices: Query<&NoticeOfEntrance>,
    players: Query<&Transform, With<Player>>,
) {
    if notices.iter().len() != 0 {
        return;
    }

    for player in &players {
        if (player.translation.x - (-450.0)).abs() < 60.0
            && (player.translation.y - (-250.0)).abs() < 60.0
        {
            spawn_arrow(
                &mut commands,
                Transform::from_xyz(-450.0, -200.0, 1.0),
                &asset_server,
                NoticeOfEntrance,
            );
            spawn_press_e(
                &mut commands,
                Transform::from_xyz(-390.0, -200.0, 1.0),
                &asset_server,
                "返回",
                NoticeOfEntrance,
            );
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfEntrance>>,
    players: Query<&Transform, With<Player>>,
) {
    for player in &players {
        if (player.translation.x - (-450.0)).abs() < 60.0
            && (player.translation.y - (-250.0)).abs() < 60.0
        {
            return;
        }
    }

    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfEntrance>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}
