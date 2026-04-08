use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, hole::Hole, player::Player, press_e::spawn_press_e};

#[derive(Component)]
pub struct NoticeOfHole;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notices: Query<&NoticeOfHole>,
    players: Query<&Transform, With<Player>>,
    lifts: Query<&Transform, With<Hole>>,
) {
    if notices.iter().len() != 0 {
        return;
    }

    for player in &players {
        for lift in &lifts {
            if (player.translation.x - lift.translation.x).abs() < 60.0 {
                // Keep the same UI offset pattern as Day2Scene3.
                let arrow_pos = Transform::from_xyz(lift.translation.x, 60.0, 1.0);
                let press_e_pos = Transform::from_xyz(lift.translation.x + 76.0, 60.0, 1.0);
                spawn_arrow(&mut commands, arrow_pos, &asset_server, NoticeOfHole);
                spawn_press_e(
                    &mut commands,
                    press_e_pos,
                    &asset_server,
                    "离开",
                    NoticeOfHole,
                );
                return;
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfHole>>,
    players: Query<&Transform, With<Player>>,
    lifts: Query<&Transform, With<Hole>>,
) {
    for player in &players {
        for lift in &lifts {
            if (player.translation.x - lift.translation.x).abs() < 60.0 {
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
