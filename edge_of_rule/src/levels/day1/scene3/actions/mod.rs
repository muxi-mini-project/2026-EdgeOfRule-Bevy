use crate::{
    core::state::GameState,
    entities::player::Player,
    levels::day1::{
        scene3::Scene3DoorState,
        scene3::spawner::{fog::Day1Scene3Fog, press_e_to_open_door::PressEtoOpenDoor},
    },
};
use bevy::prelude::*;

pub fn open_door(
    query: Query<&PressEtoOpenDoor>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(Scene3DoorState::Opened);
    }
}

pub fn enter_door(
    query: Query<&PressEtoOpenDoor>,
    input: Res<ButtonInput<KeyCode>>,
    door_state: Res<Scene3DoorState>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if *door_state != Scene3DoorState::Opened {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        game_state.set(GameState::Day1Scene4);
    }
}

pub fn fog_follow(
    mut fog: Query<&mut Transform, (With<Day1Scene3Fog>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Day1Scene3Fog>)>,
) {
    for mut fog_transform in fog.iter_mut() {
        for player_transform in player.iter() {
            fog_transform.translation = player_transform.translation;
        }
    }
}
