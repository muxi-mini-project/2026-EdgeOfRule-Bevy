use bevy::prelude::*;

use crate::{
    core::state::GameState,
    entities::player::SpawnPoint,
    levels::day1::{
        scene1::Picked,
        scene2::{
            TrapdoorState,
            spawner::{
                press_e_to_back_scene1::PressEtoBackScene1,
                press_e_to_enter_trapdoor::PressEtoEnterTrapdoor,
            },
        },
    },
};

pub fn back_to_scene1(
    mut commands: Commands,
    query: Query<&PressEtoBackScene1>,
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(408.0, -68.0, 0.0)));
        state.set(GameState::Day1Scene1);
    }
}

pub fn open_trapdoor(
    query: Query<&PressEtoEnterTrapdoor>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    picked: Res<Picked>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if *picked != Picked::Screw {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(TrapdoorState::Opened);
    }
}

pub fn enter_trapdoor(
    mut commands: Commands,
    query: Query<&PressEtoEnterTrapdoor>,
    input: Res<ButtonInput<KeyCode>>,
    trapdoor_state: Res<TrapdoorState>,
    mut state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if *trapdoor_state != TrapdoorState::Opened {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(-398.0, 250.0, 0.0)));
        state.set(GameState::Day1Scene3);
    }
}
