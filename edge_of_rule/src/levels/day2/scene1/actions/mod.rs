use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::Text2dBounds;

use crate::{
    animation::fade_mask::{FadeMask, spawn_mask},
    constants::SCALE,
    core::state::GameState,
    entities::player::{Player, SpawnPoint},
    levels::day2::scene1::spawner::notice_of_notice_board::NoticeOfNoticeBoard,
    levels::day2::scene1::spawner::notice_of_windows::NoticeOfWindow,
};

const DAY2_SCENE2_DESK0_SPAWN: Vec3 = Vec3::new(-160.0, -68.0, 0.0);
const DAY2_SCENE2_DESK1_SPAWN: Vec3 = Vec3::new(0.0, -68.0, 0.0);
const DAY2_SCENE2_DESK2_SPAWN: Vec3 = Vec3::new(160.0, -68.0, 0.0);

#[derive(Component)]
pub struct OpenedNoticeBoard;

pub fn read_small_note(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&NoticeOfNoticeBoard>,
    input: Res<ButtonInput<KeyCode>>,
    note: Query<&OpenedNoticeBoard>,
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
                    texture: asset_server.load("images/HUD/notice_board.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 5.0).with_scale(Vec3::splat(SCALE)),
                    ..default()
                },
                OpenedNoticeBoard,
            ))
            .with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        "相信教室...",
                        TextStyle {
                            font: asset_server.load("font/font/aLiFont.ttf"),
                            font_size: 8.0,
                            color: Color::RED,
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
    query: Query<Entity, With<OpenedNoticeBoard>>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn enter_scene3(
    mut commands: Commands,
    this: Res<State<GameState>>,
    players: Query<&Transform, With<Player>>,
    mask: Query<&FadeMask>,
) {
    if mask.iter().len() != 0 {
        return;
    }
    if *this.get() != GameState::Day2Scene1 {
        return;
    }
    for player in &players {
        if player.translation.x > 350.0 {
            commands.insert_resource(SpawnPoint(Transform::from_xyz(-104.0, -68.0, 0.0)));
            spawn_mask(&mut commands, GameState::Day2Scene3);
        }
    }
}

pub fn enter_scene2(
    mut commands: Commands,
    this: Res<State<GameState>>,
    notice: Query<&NoticeOfWindow>,
    input: Res<ButtonInput<KeyCode>>,
    mask: Query<&FadeMask>,
) {
    if mask.iter().len() != 0 {
        return;
    }
    if *this.get() != GameState::Day2Scene1 {
        return;
    }
    let mut idx: Option<usize> = None;
    for n in &notice {
        idx = Some(n.0);
        break;
    }
    let Some(idx) = idx else {
        return;
    };

    if input.just_pressed(KeyCode::KeyE) {
        let spawn = match idx {
            0 => DAY2_SCENE2_DESK0_SPAWN,
            1 => DAY2_SCENE2_DESK1_SPAWN,
            _ => DAY2_SCENE2_DESK2_SPAWN,
        };
        commands.insert_resource(SpawnPoint(Transform::from_translation(spawn)));
        spawn_mask(&mut commands, GameState::Day2Scene2);
    }
}
