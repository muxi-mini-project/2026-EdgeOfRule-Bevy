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
    lifts: Query<&Transform, With<Lift>>,
    lift_state: Res<LiftState>,
) {
    if notices.iter().len() != 0 {
        return;
    }
    if *lift_state == LiftState::Broken {
        return;
    }

    for player in &players {
        for lift in &lifts {
            if (player.translation.x - lift.translation.x).abs() < 60.0 {
                // Keep the same UI offset pattern as Day2Scene3.
                let arrow_pos = Transform::from_xyz(lift.translation.x, 160.0, 1.0);
                let press_e_pos = Transform::from_xyz(lift.translation.x + 76.0, 160.0, 1.0);
                spawn_arrow(&mut commands, arrow_pos, &asset_server, NoticeOfLift);
                spawn_press_e(
                    &mut commands,
                    press_e_pos,
                    &asset_server,
                    "离开",
                    NoticeOfLift,
                );
                return;
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    notices: Query<Entity, With<NoticeOfLift>>,
    players: Query<&Transform, With<Player>>,
    lifts: Query<&Transform, With<Lift>>,
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

pub fn despawn_all(mut commands: Commands, notices: Query<Entity, With<NoticeOfLift>>) {
    for notice in &notices {
        commands.entity(notice).despawn_recursive();
    }
}
