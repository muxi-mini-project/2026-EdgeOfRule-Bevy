use bevy::prelude::*;

use crate::{
    constants::SCALE,
    core::state::GameState,
    levels::day1::scene1::spawner::{
        press_e_to_open_door::PressEtoOpenDoor, press_e_to_read::PressEtoRead,
    },
};

#[derive(Component)]
pub struct OpenedSmallNote;

pub fn open_door(
    query: Query<&PressEtoOpenDoor>,
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        state.set(GameState::Day1Scene2);
    }
}

pub fn read_small_note(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&PressEtoRead>,
    input: Res<ButtonInput<KeyCode>>,
    note: Query<&OpenedSmallNote>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if note.iter().len() != 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/HUD/small_note.png"),
                transform: Transform::from_xyz(0.0, 0.0, 5.0).with_scale(Vec3::splat(SCALE)),
                ..default()
            },
            OpenedSmallNote,
        ));
    }
}

pub fn close_small_note(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<OpenedSmallNote>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
