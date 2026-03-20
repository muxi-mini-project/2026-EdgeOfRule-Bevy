use bevy::prelude::*;

use crate::{
    assets::door::DoorAssets,
    core::state::GameState,
    entities::door::Door,
    levels::day1::{scene1::Scene1DoorState, scene3::Scene3DoorState},
};

pub fn scene1_door_animation_system(
    door_state: Res<Scene1DoorState>,
    mut doors: Query<&mut Handle<Image>, With<Door>>,
    asset_server: Res<DoorAssets>,
    game_state: Res<State<GameState>>,
) {
    for mut texture in &mut doors {
        *texture = match *door_state {
            Scene1DoorState::Closed => asset_server.scene1.clone(),
            Scene1DoorState::Opened => match **game_state {
                GameState::Day1Scene1 => asset_server.scene1_opened.clone(),
                _ => asset_server.scene2.clone(),
            },
        }
    }
}

pub fn scene3_door_animation_system(
    door_state: Res<Scene3DoorState>,
    mut doors: Query<&mut Handle<Image>, With<Door>>,
    asset_server: Res<DoorAssets>,
) {
    for mut texture in &mut doors {
        *texture = match *door_state {
            Scene3DoorState::Closed => asset_server.scene3.clone(),
            Scene3DoorState::Opened => asset_server.scene3_opened.clone(),
        }
    }
}
