use crate::{
    core::state::GameState,
    levels::day1::{scene1::DoorState, scene3::spawner::press_e_to_open_door::PressEtoOpenDoor},
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
        commands.insert_resource(DoorState::Opened);
    }
}

pub fn enter_door(
    query: Query<&PressEtoOpenDoor>,
    input: Res<ButtonInput<KeyCode>>,
    door_state: Res<DoorState>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if *door_state != DoorState::Opened {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        game_state.set(GameState::Day1Scene4);
    }
}
