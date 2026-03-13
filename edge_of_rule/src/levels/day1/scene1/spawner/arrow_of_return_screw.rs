use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, player::Player, screw::Screw},
    levels::day1::scene1::Picked,
};

#[derive(Component)]
pub struct ArrowOfReturnScrew;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfReturnScrew>,
    players: Query<&Transform, With<Player>>,
    picked: Res<Picked>,
) {
    if *picked != Picked::Screw {
        return;
    }
    if arrows.iter().len() != 0 {
        return;
    }

    for player in &players {
        if (player.translation.x - (-396.0)).abs() < 60.0 {
            spawn_arrow(
                &mut commands,
                Transform::from_xyz(-396.0, -64.0 + 60.0, 1.0),
                &asset_server,
                ArrowOfReturnScrew,
            );
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfReturnScrew>>,
    players: Query<&Transform, With<Player>>,
    screws: Query<&Screw>,
) {
    for player in &players {
        if (player.translation.x - (-396.0)).abs() < 60.0 && screws.iter().len() == 0 {
            return;
        }
    }

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfReturnScrew>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
