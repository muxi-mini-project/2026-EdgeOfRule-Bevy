use bevy::prelude::*;

use crate::{constants::SCALE, entities::windows::Windows};

#[derive(Component)]
pub struct Day2Scene5OfficeWindows;

// Hard-coded placement.
const POS: Vec3 = Vec3::new(224.0, 24.0, -4.0);

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset.load("images/animations/windows.png"),
            transform: Transform::from_translation(POS).with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Windows,
        Day2Scene5OfficeWindows,
    ));
}

pub fn despawn(mut commands: Commands, windows: Query<Entity, With<Day2Scene5OfficeWindows>>) {
    for e in &windows {
        commands.entity(e).despawn_recursive();
    }
}
