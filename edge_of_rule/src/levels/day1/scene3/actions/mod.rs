use crate::{
    core::state::GameState,
    entities::{
        elevator::Elevator,
        player::{Player, SpawnPoint},
    },
    levels::day1::scene3::{
        Scene3ChestState, Scene3CoverState, Scene3DoorState,
        spawner::{
            fog::Day1Scene3Fog, press_e_to_enter_hole::PressEtoEnterHole,
            press_e_to_open_chest::PressEtoOpenChest, press_e_to_open_cover::PressEtoOpenCover,
            press_e_to_open_door::PressEtoOpenDoor,
        },
    },
};
use bevy::prelude::*;

pub fn enter_hole(
    mut commands: Commands,
    query: Query<&PressEtoEnterHole>,
    input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(116.0, -68.0, 0.0)));
        game_state.set(GameState::Day1Scene2);
    }
}

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
    mut commands: Commands,
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
        commands.insert_resource(SpawnPoint(Transform::from_xyz(-92.0, -68.0, 0.0)));
        game_state.set(GameState::Day1Scene4);
    }
}

pub fn open_chest(
    querys: Query<&PressEtoOpenChest>,
    input: Res<ButtonInput<KeyCode>>,
    mut chest_state: ResMut<Scene3ChestState>,
) {
    if querys.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        match *chest_state {
            Scene3ChestState::Packed => *chest_state = Scene3ChestState::Opened,
            Scene3ChestState::Opened => *chest_state = Scene3ChestState::Picked,
            _ => (),
        }
    }
}

pub fn open_cover(
    querys: Query<&PressEtoOpenCover>,
    input: Res<ButtonInput<KeyCode>>,
    mut chest_state: ResMut<Scene3CoverState>,
) {
    if querys.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        match *chest_state {
            Scene3CoverState::Packed => *chest_state = Scene3CoverState::Opened,
            Scene3CoverState::Opened => *chest_state = Scene3CoverState::Picked,
            _ => (),
        }
    }
}

pub fn fog_follow(
    mut fog: Query<&mut Transform, (With<Day1Scene3Fog>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Day1Scene3Fog>)>,
) {
    for mut fog_transform in fog.iter_mut() {
        for player_transform in player.iter() {
            fog_transform.translation = player_transform.translation;
            fog_transform.translation.z += 20.0;
        }
    }
}

pub fn move_elevator_when_chest_picked(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    chest_state: Res<Scene3ChestState>,
    mut elevators: Query<(&mut Transform, &Elevator)>,
) {
    if *chest_state != Scene3ChestState::Picked {
        return;
    }

    let mut dir = 0.0;
    if input.pressed(KeyCode::ArrowUp) {
        dir += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        dir -= 1.0;
    }

    if dir == 0.0 {
        return;
    }

    const ELEVATOR_SPEED: f32 = 60.0;
    let delta_y = dir * ELEVATOR_SPEED * time.delta_seconds();
    for (mut transform, elevator) in &mut elevators {
        transform.translation.y =
            (transform.translation.y + delta_y).clamp(elevator.min_y, elevator.max_y);
    }
}
