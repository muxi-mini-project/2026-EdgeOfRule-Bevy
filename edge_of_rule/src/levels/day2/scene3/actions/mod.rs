use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{
    animation::fade_mask::{FadeMask, spawn_mask},
    constants::SCALE,
    core::state::GameState,
    entities::player::{Player, SpawnPoint},
    levels::day2::scene3::spawner::{
        notice_of_circuit::NoticeOfCircuit, notice_of_hole::NoticeOfHole,
    },
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

#[derive(Resource, Eq, PartialEq)]
pub enum LiftState {
    Broken,
    Fixed,
}

#[derive(Resource, Eq, PartialEq, Default)]
pub struct Buttons(pub [[bool; 5]; 5]);

#[derive(Component)]
pub struct Button(pub usize, pub usize);

#[derive(Component)]
pub struct OpenedCircuit;

pub fn check_circuit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&NoticeOfCircuit>,
    input: Res<ButtonInput<KeyCode>>,
    note: Query<&OpenedCircuit>,
    buttons: Res<Buttons>,
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
                texture: asset_server.load("images/HUD/circuit.png"),
                transform: Transform::from_xyz(0.0, 0.0, 10.0).with_scale(Vec3::splat(SCALE)),
                ..default()
            },
            OpenedCircuit,
        ));

        for i in 0..5 {
            for j in 0..5 {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: if buttons.0[i][j] {
                                Color::WHITE
                            } else {
                                Color::GRAY
                            },
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            (j as f32) * 52.0 - 78.0,
                            -(i as f32) * 52.0 + 78.0,
                            15.0,
                        )
                        .with_scale(Vec3::splat(SCALE)),
                        ..Default::default()
                    },
                    Button(i, j),
                    OpenedCircuit,
                ));
            }
        }
    }
}

pub fn close_circuit(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<OpenedCircuit>>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn toggle_button(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut buttons: ResMut<Buttons>,
    button_query: Query<(&Transform, &Sprite, &Button), With<OpenedCircuit>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();
    let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    for (transform, sprite, button) in &button_query {
        let size = sprite.custom_size.unwrap_or(Vec2::ZERO);
        let scale = Vec2::new(transform.scale.x, transform.scale.y);
        let half = (size * scale) * 0.5;
        let center = transform.translation.truncate();

        let hit =
            (world_pos.x - center.x).abs() <= half.x && (world_pos.y - center.y).abs() <= half.y;
        if !hit {
            continue;
        }

        let i = button.0;
        let j = button.1;
        buttons.0[i][j] = !buttons.0[i][j];
        break;
    }
}

pub fn update_button(
    buttons: Res<Buttons>,
    mut button_query: Query<(&Button, &mut Sprite), With<OpenedCircuit>>,
) {
    if !buttons.is_changed() {
        return;
    }

    for (button, mut sprite) in &mut button_query {
        let i = button.0;
        let j = button.1;
        sprite.color = if buttons.0[i][j] {
            Color::WHITE
        } else {
            Color::GRAY
        };
    }
}

pub fn update_lift(buttons: Res<Buttons>, mut lift_state: ResMut<LiftState>) {
    if !buttons.is_changed() {
        return;
    }

    const TARGET: [[bool; 5]; 5] = [
        [true, true, true, false, true],
        [false, false, true, true, false],
        [true, true, true, true, true],
        [true, true, false, false, true],
        [false, true, false, true, false],
    ];

    let fixed = buttons.0 == TARGET;
    if fixed {
        if *lift_state != LiftState::Fixed {
            *lift_state = LiftState::Fixed;
        }
    } else if *lift_state != LiftState::Broken {
        *lift_state = LiftState::Broken;
    }
}
