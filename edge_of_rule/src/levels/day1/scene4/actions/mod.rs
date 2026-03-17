use bevy::prelude::*;

use crate::{
    animation::fade_mask::spawn_mask,
    constants::SCALE,
    core::state::GameState,
    entities::player::{Player, SpawnPoint},
    levels::day1::{
        scene1::Day1Finished,
        scene4::spawner::{
            fog::Day1Scene4Fog, press_e_to_enter::PressEtoEnter, press_e_to_read_log::PressEtoRead,
        },
    },
};

pub fn fog_follow(
    mut fog: Query<&mut Transform, (With<Day1Scene4Fog>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Day1Scene4Fog>)>,
) {
    for mut fog_transform in fog.iter_mut() {
        for player_transform in player.iter() {
            fog_transform.translation = player_transform.translation;
            fog_transform.translation.z += 20.0;
        }
    }
}

pub fn back_to_scene3(
    mut commands: Commands,
    query: Query<&PressEtoEnter>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(394.0, 100.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day1Scene3);
    }
}

#[derive(Component)]
pub struct OpenedLog;

pub fn read_log(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&PressEtoRead>,
    input: Res<ButtonInput<KeyCode>>,
    note: Query<&OpenedLog>,
    mut finished: ResMut<Day1Finished>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if note.iter().len() != 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/HUD/small_note.png"),
                transform: Transform::from_xyz(0.0, 0.0, 50.0).with_scale(Vec3::splat(SCALE)),
                ..default()
            },
            OpenedLog,
        ));
        *finished = Day1Finished::Yes;
    }
}

pub fn close_log(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<OpenedLog>>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
