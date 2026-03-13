use bevy::prelude::*;

use crate::{assets::levels::LevelsImageAssets, constants::SCALE};

#[derive(Component)]
pub struct Day1Scene4Fog;

pub fn spawn(mut commands: Commands, assets: Res<LevelsImageAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.day1_scene3_fog.clone(),
            transform: Transform::from_xyz(-234.0, 250.0, 20.0).with_scale(Vec3::splat(SCALE)),
            ..Default::default()
        },
        Day1Scene4Fog,
    ));
}

pub fn despawn(mut commands: Commands, backgrounds: Query<Entity, With<Day1Scene4Fog>>) {
    for background in &backgrounds {
        commands.entity(background).despawn();
    }
}
