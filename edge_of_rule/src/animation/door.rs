use bevy::prelude::*;

use crate::{
    core::state::GameState,
    entities::door::Door,
    levels::day1::{scene1::Scene1DoorState, scene3::Scene3DoorState},
};

pub fn scene1_door_animation_system(
    door_state: Res<Scene1DoorState>,
    mut doors: Query<&mut Handle<Image>, With<Door>>,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
) {
    for mut texture in &mut doors {
        *texture = match *door_state {
            Scene1DoorState::Closed => asset_server.load("images/animations/door.png"),
            Scene1DoorState::Opened => match **game_state {
                GameState::Day1Scene1 => {
                    asset_server.load("images/levels/day1/scene1_opened_door.png")
                }
                _ => asset_server.load("images/levels/day1/scene2_opened_door.png"),
            },
        }
    }
}

pub fn scene3_door_animation_system(
    door_state: Res<Scene3DoorState>,
    mut doors: Query<&mut Handle<Image>, With<Door>>,
    asset_server: Res<AssetServer>,
) {
    for mut texture in &mut doors {
        *texture = match *door_state {
            Scene3DoorState::Closed => asset_server.load("images/animations/control_room_door.png"),
            Scene3DoorState::Opened => {
                asset_server.load("images/levels/day1/scene3_opened_door.png")
            }
        }
    }
}
