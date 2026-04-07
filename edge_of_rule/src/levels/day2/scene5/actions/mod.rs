use bevy::prelude::*;

use crate::{
    animation::fade_mask::spawn_mask, animation::lift_door::LiftDoorAnim, core::state::GameState,
    entities::player::SpawnPoint, levels::day2::scene5::spawner::notice_of_lift::NoticeOfLift,
};

pub fn start_close_lift_on_enter(mut lift_door: ResMut<LiftDoorAnim>) {
    lift_door.start_close();
}

pub fn enter_lift(
    mut commands: Commands,
    query: Query<&NoticeOfLift>,
    input: Res<ButtonInput<KeyCode>>,
    mut lift_door: ResMut<LiftDoorAnim>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        // Back to Day2Scene3 and spawn right in front of its lift.
        lift_door.start_open();
        commands.insert_resource(SpawnPoint(Transform::from_xyz(-48.0, -68.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day2Scene3);
    }
}
