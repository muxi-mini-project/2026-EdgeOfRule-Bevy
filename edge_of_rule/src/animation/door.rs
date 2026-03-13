use bevy::prelude::*;

use crate::{core::state::GameState, entities::door::Door, levels::day1::scene1::DoorState};

pub fn door_animation_system(
    door_state: Res<DoorState>,
    game_state: Res<State<GameState>>,
    mut doors: Query<&mut Handle<Image>, With<Door>>,
    asset_server: Res<AssetServer>,
) {
    for mut texture in &mut doors {
        *texture = match *door_state {
            DoorState::Closed => match **game_state {
                GameState::Day1Scene3 => {
                    asset_server.load("images/animations/control_room_door.png")
                }
                _ => asset_server.load("images/animations/door.png"),
            },
            DoorState::Opened => match **game_state {
                GameState::Day1Scene1 => {
                    asset_server.load("images/levels/day1/scene1_opened_door.png")
                }
                GameState::Day1Scene2 => {
                    asset_server.load("images/levels/day1/scene2_opened_door.png")
                }
                _ => asset_server.load("images/animations/door.png"),
            },
        }
    }
}
