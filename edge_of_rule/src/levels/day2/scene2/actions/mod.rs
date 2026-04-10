use bevy::prelude::*;

use crate::{
    animation::fade_mask::{spawn_mask, FadeMask},
    core::state::GameState,
    entities::player::SpawnPoint,
    levels::day2::scene2::spawner::notice_of_door::{NoticeOfLeftDoor, NoticeOfRightDoor},
};

const DAY2_SCENE1_LEFT_DOOR_SPAWN: Vec3 = Vec3::new(-200.0, -68.0, 0.0);
const DAY2_SCENE1_RIGHT_DOOR_SPAWN: Vec3 = Vec3::new(200.0, -68.0, 0.0);

pub fn exit_left_door(
    mut commands: Commands,
    query: Query<&NoticeOfLeftDoor>,
    input: Res<ButtonInput<KeyCode>>,
    mask: Query<&FadeMask>,
) {
    if mask.iter().len() != 0 {
        return;
    }

    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_translation(
            DAY2_SCENE1_LEFT_DOOR_SPAWN,
        )));
        spawn_mask(&mut commands, GameState::Day2Scene1);
    }
}

pub fn exit_right_door(
    mut commands: Commands,
    query: Query<&NoticeOfRightDoor>,
    input: Res<ButtonInput<KeyCode>>,
    mask: Query<&FadeMask>,
) {
    if mask.iter().len() != 0 {
        return;
    }

    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_translation(
            DAY2_SCENE1_RIGHT_DOOR_SPAWN,
        )));
        spawn_mask(&mut commands, GameState::Day2Scene1);
    }
}
