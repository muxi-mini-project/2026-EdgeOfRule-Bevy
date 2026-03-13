use bevy::prelude::*;

use crate::{assets::levels::LevelsImageAssets, constants::SCALE};

#[derive(Component)]
pub struct Day1Scene4Background;

pub fn spawn(mut commands: Commands, assets: Res<LevelsImageAssets>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2 { x: 320.0, y: 200.0 }),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -20.0).with_scale(Vec3::splat(SCALE)),
            ..Default::default()
        },
        Day1Scene4Background,
    ));
    commands.spawn((
        SpriteBundle {
            texture: assets.day1_scene4_background.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -10.0).with_scale(Vec3::splat(SCALE)),
            ..Default::default()
        },
        Day1Scene4Background,
    ));
}

pub fn despawn(mut commands: Commands, backgrounds: Query<Entity, With<Day1Scene4Background>>) {
    for background in &backgrounds {
        commands.entity(background).despawn();
    }
}
