use bevy::prelude::*;

use crate::{
    constants::SCALE,
    core::state::GameState,
    entities::{
        key::{Key, spawn_key},
        screw::{Screw, spawn_screw},
    },
    levels::day1::scene1::{
        Picked,
        spawner::{
            press_e_to_open_door::PressEtoOpenDoor, press_e_to_pick_key::PressEtoPickKey,
            press_e_to_pick_screw::PressEtoPickScrew, press_e_to_read::PressEtoRead,
            press_e_to_return_key::PressEtoReturnKey, press_e_to_return_screw::PressEtoReturnScrew,
        },
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

pub fn pick_key(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&PressEtoPickKey>,
    keys: Query<Entity, With<Key>>,
    mut picked: ResMut<Picked>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        for key in keys.iter() {
            commands.entity(key).despawn();
            *picked = Picked::Key;
        }
    }
}

pub fn pick_screw(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&PressEtoPickScrew>,
    screws: Query<Entity, With<Screw>>,
    mut picked: ResMut<Picked>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        for screw in screws.iter() {
            commands.entity(screw).despawn();
            *picked = Picked::Screw;
        }
    }
}

pub fn return_key(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&PressEtoReturnKey>,
    mut picked: ResMut<Picked>,
    asset: Res<AssetServer>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if *picked != Picked::Key {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        spawn_key(
            &mut commands,
            Transform::from_xyz(-220.0, -64.0, -5.0),
            asset,
        );

        *picked = Picked::None
    }
}

pub fn return_screw(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&PressEtoReturnScrew>,
    mut picked: ResMut<Picked>,
    asset: Res<AssetServer>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if *picked != Picked::Screw {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        spawn_screw(
            &mut commands,
            Transform::from_xyz(-396.0, -64.0, -5.0),
            asset,
        );

        *picked = Picked::None
    }
}
