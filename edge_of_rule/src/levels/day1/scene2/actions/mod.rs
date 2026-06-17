use bevy::prelude::*;

use crate::{
    animation::fade_mask::spawn_mask,
    constants::SCALE,
    core::{inventory::{Inventory, InventoryItem}, state::GameState},
    entities::player::SpawnPoint,
    levels::day1::scene2::{
        spawner::{
            press_e_to_back_scene1::PressEtoBackScene1,
            press_e_to_enter_trapdoor::PressEtoEnterTrapdoor,
            press_e_to_look_mirror::PressEtoLookMirror,
        },
        TrapdoorState,
    },
};

#[derive(Component)]
pub struct OpenedMirror;

pub fn back_to_scene1(
    mut commands: Commands,
    query: Query<&PressEtoBackScene1>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(408.0, -68.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day1Scene1);
    }
}

pub fn open_trapdoor(
    query: Query<&PressEtoEnterTrapdoor>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    inventory: Res<Inventory>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if inventory.selected_item() != Some(InventoryItem::Screw) {
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
) {
    if query.iter().len() == 0 {
        return;
    }

    if *trapdoor_state != TrapdoorState::Opened {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(-398.0, 250.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day1Scene3);
    }
}

pub fn look_mirror(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&PressEtoLookMirror>,
    input: Res<ButtonInput<KeyCode>>,
    mirror: Query<&OpenedMirror>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if mirror.iter().len() != 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/HUD/mirror.png"),
                transform: Transform::from_xyz(0.0, 0.0, 5.0).with_scale(Vec3::splat(SCALE)),
                ..default()
            },
            OpenedMirror,
        ));
    }
}

pub fn close_mirror(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<OpenedMirror>>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
