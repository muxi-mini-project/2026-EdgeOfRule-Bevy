use bevy::prelude::*;

use crate::{
    entities::{cover::Cover, player::Player, press_e::spawn_press_e},
    levels::day1::scene3::Scene3CoverState,
};

#[derive(Component)]
pub struct PressEtoOpenCover;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    querys: Query<&PressEtoOpenCover>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Cover>>,
    chest_state: Res<Scene3CoverState>,
) {
    if querys.iter().len() != 0 && !chest_state.is_changed() {
        return;
    }

    if *chest_state == Scene3CoverState::Picked {
        return;
    }
    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0
                && (player.translation.y - door.translation.y).abs() < 90.0
            {
                spawn_press_e(
                    &mut commands,
                    Transform::from_xyz(-254.0, -38.0, 25.0),
                    &asset_server,
                    if *chest_state == Scene3CoverState::Packed {
                        "打开"
                    } else {
                        "拾取钥匙"
                    },
                    PressEtoOpenCover,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    querys: Query<Entity, With<PressEtoOpenCover>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Cover>>,
    chest_state: Res<Scene3CoverState>,
) {
    for player in &players {
        if *chest_state == Scene3CoverState::Picked || chest_state.is_changed() {
            break;
        }
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0
                && (player.translation.y - door.translation.y).abs() < 90.0
            {
                return;
            }
        }
    }

    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}

pub fn despawn_all(mut commands: Commands, querys: Query<Entity, With<PressEtoOpenCover>>) {
    for query in &querys {
        commands.entity(query).despawn_recursive();
    }
}
