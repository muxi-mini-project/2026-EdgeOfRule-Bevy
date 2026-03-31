use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct Mask;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/levels/day2/scene1_mask.png"),
            transform: Transform::from_xyz(448.0, 0.0, 5.0).with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Mask,
    ));
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<Mask>>) {
    commands.entity(query.get_single().unwrap()).despawn();
}
