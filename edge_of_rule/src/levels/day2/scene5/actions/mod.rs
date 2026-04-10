use bevy::prelude::*;

use crate::{
    animation::{fade_mask::spawn_mask, lift_door::LiftDoorAnim},
    core::state::GameState,
    entities::player::SpawnPoint,
    levels::day2::scene5::spawner::{
        notice_of_office_log::NoticeOfOfficeLog, notice_of_office_windows::NoticeOfOfficeWindows,
    },
    levels::day2::{
        scene3::ExitOn,
        scene5::spawner::{notice_of_hole::NoticeOfHole, notice_of_lift::NoticeOfLift},
    },
};

use bevy::sprite::Anchor;
use bevy::text::Text2dBounds;

use crate::constants::SCALE;

pub fn start_close_lift_on_enter(mut lift_door: ResMut<LiftDoorAnim>) {
    lift_door.start_close();
}

pub fn reset_lift_door_on_enter(mut lift_door: ResMut<LiftDoorAnim>) {
    // Entering from the hole should not trigger any lift door animation.
    *lift_door = LiftDoorAnim::default();
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
        commands.insert_resource(ExitOn::Lift);
        spawn_mask(&mut commands, GameState::Day2Scene3);
    }
}

pub fn back_to_scene4(
    mut commands: Commands,
    notice: Query<&NoticeOfHole>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if notice.iter().len() == 0 {
        return;
    }

    if input.just_pressed(KeyCode::KeyE) {
        commands.insert_resource(SpawnPoint(Transform::from_xyz(450.0, 250.0, 0.0)));
        spawn_mask(&mut commands, GameState::Day2Scene4);
    }
}

#[derive(Component)]
pub struct OpenedOfficeLog;

pub fn read_office_log(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notice: Query<&NoticeOfOfficeLog>,
    input: Res<ButtonInput<KeyCode>>,
    opened: Query<&OpenedOfficeLog>,
) {
    if notice.iter().len() == 0 {
        return;
    }
    if opened.iter().len() != 0 {
        return;
    }
    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }

    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("images/HUD/small_note.png"),
                transform: Transform::from_xyz(0.0, 0.0, 50.0).with_scale(Vec3::splat(SCALE)),
                ..default()
            },
            OpenedOfficeLog,
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    "翻过，坠落",
                    TextStyle {
                        font: asset_server.load("font/font/aLiFont.ttf"),
                        font_size: 12.0,
                        color: Color::BLACK,
                    },
                )
                .with_justify(JustifyText::Center),
                text_anchor: Anchor::Center,
                text_2d_bounds: Text2dBounds {
                    size: Vec2::new(200.0, f32::INFINITY),
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });
        });
}

pub fn close_office_log(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    opened: Query<Entity, With<OpenedOfficeLog>>,
) {
    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }
    for e in &opened {
        commands.entity(e).despawn_recursive();
    }
}

pub fn flip_over_windows_to_day3(
    mut commands: Commands,
    notice: Query<&NoticeOfOfficeWindows>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if notice.iter().len() == 0 {
        return;
    }
    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }
    commands.insert_resource(SpawnPoint(Transform::from_translation(Vec3::ZERO)));
    spawn_mask(&mut commands, GameState::Day3);
}
