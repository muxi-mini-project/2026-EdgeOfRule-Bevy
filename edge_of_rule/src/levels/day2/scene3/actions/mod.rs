use bevy::prelude::*;

use crate::{
    animation::fade_mask::{FadeMask, spawn_mask},
    core::state::GameState,
    entities::player::{Player, SpawnPoint},
    levels::day2::scene3::spawner::notice_of_hole::NoticeOfHole,
};

pub fn enter_scene1(
    mut commands: Commands,
    this: Res<State<GameState>>,
    players: Query<&Transform, With<Player>>,
    mask: Query<&FadeMask>,
) {
    if mask.iter().len() != 0 {
        return;
    }
    if *this.get() != GameState::Day2Scene3 {
        return;
    }
    for player in &players {
        if player.translation.x < -143.0 {
            commands.insert_resource(SpawnPoint(Transform::from_xyz(312.0, -68.0, 0.0)));
            spawn_mask(&mut commands, GameState::Day2Scene1);
        }
    }
}

pub fn enter_hole(
    mut commands: Commands,
    query: Query<&NoticeOfHole>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(-450.0, -250.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day2Scene4);
    }
}
