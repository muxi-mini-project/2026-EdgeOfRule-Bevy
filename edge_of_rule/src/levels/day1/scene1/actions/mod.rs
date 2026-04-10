use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::Text2dBounds;

use crate::{
    animation::fade_mask::spawn_mask,
    constants::SCALE,
    core::state::GameState,
    entities::{
        key::{spawn_key, Key},
        player::SpawnPoint,
        screw::{spawn_screw, Screw},
    },
    levels::day1::scene1::{
        spawner::{
            press_e_to_open_door::PressEtoOpenDoor, press_e_to_pick_key::PressEtoPickKey,
            press_e_to_pick_screw::PressEtoPickScrew, press_e_to_read::PressEtoRead,
            press_e_to_return_key::PressEtoReturnKey, press_e_to_return_screw::PressEtoReturnScrew,
            press_e_to_sleep::PressEtoSleep,
        },
        Picked, Scene1DoorState,
    },
    ui::mainmenu::spawner::lock_levels::{
        LevelTwoLock,    
    },
};

#[derive(Component)]
pub struct OpenedSmallNote;

pub fn open_door(
    query: Query<&PressEtoOpenDoor>,
    input: Res<ButtonInput<KeyCode>>,
    picked: Res<Picked>,
    mut commands: Commands,
) {
    if query.iter().len() == 0 {
        return;
    }

    if *picked != Picked::Key {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(Scene1DoorState::Opened);
    }
}

pub fn enter_door(
    mut commands: Commands,
    query: Query<&PressEtoOpenDoor>,
    input: Res<ButtonInput<KeyCode>>,
    door_state: Res<Scene1DoorState>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if *door_state != Scene1DoorState::Opened {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(-92.0, -50.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day1Scene2);
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
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("images/HUD/small_note.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 5.0).with_scale(Vec3::splat(SCALE)),
                    ..default()
                },
                OpenedSmallNote,
            ))
            .with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        "控制室里\n或许有线索",
                        TextStyle {
                            font: asset_server.load("font/font/aLiFont.ttf"),
                            font_size: 12.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_justify(JustifyText::Center),
                    text_anchor: Anchor::Center,
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(200.0, 120.0),
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 6.0),
                    ..default()
                });
            });
    }
}

pub fn close_small_note(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<OpenedSmallNote>>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
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

pub fn sleep(
    mut commands: Commands,
    query: Query<&PressEtoSleep>,
    input: Res<ButtonInput<KeyCode>>,
    mut unlock: ResMut<LevelTwoLock>,
) {
    if query.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(-292.0, -50.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day2Scene1);
        unlock.locked = false;
    }
}
