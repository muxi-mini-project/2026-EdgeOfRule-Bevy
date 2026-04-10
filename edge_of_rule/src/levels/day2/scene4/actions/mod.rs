use bevy::prelude::*;

use crate::{
    animation::fade_mask::spawn_mask,
    core::state::GameState,
    entities::player::{Player, SpawnPoint},
    levels::day2::{
        scene3::ExitOn,
        scene4::spawner::{
            fog::Day2Scene4Fog,
            notice_of_entrance::{NoticeOfEntrance, NoticeOfExit},
        },
    },
};

pub fn back_to_scene3(
    mut commands: Commands,
    notice: Query<&NoticeOfEntrance>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if notice.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(104.0, -68.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day2Scene3);
    }
}
pub fn enter_hole(
    mut commands: Commands,
    query: Query<&NoticeOfExit>,
    input: Res<ButtonInput<KeyCode>>,
    mut exit_on: ResMut<ExitOn>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(-116.0, -68.0, 0.0)));
        *exit_on = ExitOn::Hole;
        spawn_mask(&mut commands, GameState::Day2Scene5);
    }
}

pub fn fog_follow(
    mut fog: Query<&mut Transform, (With<Day2Scene4Fog>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Day2Scene4Fog>)>,
) {
    for mut fog_transform in fog.iter_mut() {
        for player_transform in player.iter() {
            fog_transform.translation = player_transform.translation;
            fog_transform.translation.z += 20.0;
        }
    }
}
