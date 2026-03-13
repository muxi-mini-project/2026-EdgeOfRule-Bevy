use bevy::prelude::*;

use crate::{
    core::state::GameState,
    entities::player::Player,
    levels::day1::scene4::spawner::{fog::Day1Scene4Fog, press_e_to_enter::PressEtoEnter},
};

pub fn fog_follow(
    mut fog: Query<&mut Transform, (With<Day1Scene4Fog>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Day1Scene4Fog>)>,
) {
    for mut fog_transform in fog.iter_mut() {
        for player_transform in player.iter() {
            fog_transform.translation = player_transform.translation;
        }
    }
}

pub fn back_to_scene3(
    query: Query<&PressEtoEnter>,
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        state.set(GameState::Day1Scene3);
    }
}
