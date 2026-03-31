use bevy::prelude::*;

use crate::{
    animation::fade_mask::spawn_mask, core::state::GameState, entities::player::SpawnPoint,
    levels::day2::scene4::spawner::notice_of_entrance::NoticeOfEntrance,
};

pub fn back_to_scene3(
    mut commands: Commands,
    notice: Query<&NoticeOfEntrance>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if notice.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(104.0, -68.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day2Scene3);
    }
}
