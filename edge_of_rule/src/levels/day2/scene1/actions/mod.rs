use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::Text2dBounds;

use crate::{
    animation::fade_mask::{FadeMask, spawn_mask},
    constants::SCALE,
    core::state::GameState,
    entities::player::Player,
    levels::day2::scene1::spawner::notice_of_notice_board::NoticeOfNoticeBoard,
};

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
                        "别走回头路...",
                        TextStyle {
                            font: asset_server.load("font/font/aLiFont.ttf"),
                            font_size: 8.0,
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
            spawn_mask(&mut commands, GameState::Day1Scene1);
        }
    }
}
