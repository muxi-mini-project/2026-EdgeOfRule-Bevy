use bevy::prelude::*;

use crate::assets::lift::LiftAssets;
use crate::constants::SCALE;

#[derive(Component)]
pub struct Lift;

pub fn spawn_lift(commands: &mut Commands, transform: Transform, assets: &Res<LiftAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.broken.clone(),
            transform: transform.with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Lift,
    ));
}
