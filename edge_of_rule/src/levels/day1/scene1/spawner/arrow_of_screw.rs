use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, player::Player, screw::Screw},
    levels::day1::scene1::Picked,
};

#[derive(Component)]
pub struct ArrowOfScrew;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfScrew>,
    players: Query<&Transform, With<Player>>,
    screws: Query<&Transform, With<Screw>>,
    picked: Res<Picked>,
) {
    if *picked != Picked::None {
        return;
    }
    if arrows.iter().len() != 0 {
        return;
    }

    for player in &players {
        for screw in &screws {
            if (player.translation.x - screw.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(screw.translation.x, screw.translation.y + 60.0, 1.0),
                    &asset_server,
                    ArrowOfScrew,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfScrew>>,
    players: Query<&Transform, With<Player>>,
    screws: Query<&Transform, With<Screw>>,
) {
    for player in &players {
        for screw in &screws {
            if (player.translation.x - screw.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfScrew>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
