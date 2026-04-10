use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, player::Player, press_e::spawn_press_e};

use super::windows::WINDOWS_POSITIONS;

#[derive(Component)]
pub struct NoticeOfWindow(pub usize);

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices: Query<&NoticeOfWindow>,
    players: Query<&Transform, With<Player>>,
) {
    if notices.iter().len() != 0 {
        return;
    }

    for player in &players {
        for (idx, x) in WINDOWS_POSITIONS.iter().copied().enumerate() {
            if (player.translation.x - x).abs() < 30.0 {
                let arrow_pos = Transform::from_xyz(x, 60.0, 1.0);
                let press_e_pos = Transform::from_xyz(x + 76.0, 60.0, 1.0);
                spawn_arrow(&mut commands, arrow_pos, &asset_server, NoticeOfWindow(idx));
                spawn_press_e(
                    &mut commands,
                    press_e_pos,
                    &asset_server,
                    "进入",
                    NoticeOfWindow(idx),
                );
                return;
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfWindow>>,
    players: Query<&Transform, With<Player>>,
) {
    for player in &players {
        for x in WINDOWS_POSITIONS {
            if (player.translation.x - x).abs() < 30.0 {
                return;
            }
        }
    }

    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfWindow>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}
