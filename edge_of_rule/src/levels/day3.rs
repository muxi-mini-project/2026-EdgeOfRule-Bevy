use bevy::prelude::*;

use crate::{assets::levels::LevelsImageAssets, core::state::GameState};

#[derive(Component)]
struct Day3Background;

#[derive(Component)]
struct Day3Text;

pub struct Day3Plugin;

impl Plugin for Day3Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Day3), spawn_background)
            .add_systems(OnEnter(GameState::Day3), spawn_text.after(spawn_background))
            .add_systems(OnExit(GameState::Day3), despawn_all);
    }
}

fn spawn_background(mut commands: Commands, assets: Res<LevelsImageAssets>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            image: UiImage::new(assets.day3_background.clone()),
            z_index: ZIndex::Global(-1),
            ..Default::default()
        },
        Day3Background,
    ));
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                z_index: ZIndex::Global(0),
                ..Default::default()
            },
            Day3Text,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "To be continued",
                TextStyle {
                    font: asset_server.load("font/font/aLiFont.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn despawn_all(
    mut commands: Commands,
    backgrounds: Query<Entity, With<Day3Background>>,
    texts: Query<Entity, With<Day3Text>>,
) {
    for entity in &backgrounds {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &texts {
        commands.entity(entity).despawn_recursive();
    }
}
