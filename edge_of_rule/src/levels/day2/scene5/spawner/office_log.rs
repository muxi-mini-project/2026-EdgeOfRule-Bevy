use bevy::prelude::*;

use crate::{constants::SCALE, entities::log::Log};

#[derive(Component)]
pub struct Day2Scene5OfficeLog;

// Hard-coded placement.
const POS: Vec3 = Vec3::new(64.0, -18.0, -3.0);

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset.load("images/animations/log.png"),
            transform: Transform::from_translation(POS).with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Log,
        Day2Scene5OfficeLog,
    ));
}

pub fn despawn(mut commands: Commands, logs: Query<Entity, With<Day2Scene5OfficeLog>>) {
    for e in &logs {
        commands.entity(e).despawn_recursive();
    }
}
