use bevy::prelude::*;

use crate::{
    entities::{arrow::spawn_arrow, player::Player, trapdoor::Trapdoor},
    levels::day1::{scene1::Picked, scene2::TrapdoorState},
};

#[derive(Component)]
pub struct ArrowOfTrapdoor;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfTrapdoor>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Trapdoor>>,
    picked: Res<Picked>,
    trapdoor_state: Res<TrapdoorState>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    if *picked != Picked::Screw && *trapdoor_state == TrapdoorState::Closed {
        return;
    }

    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(116.0, -100.0, 1.0),
                    &asset_server,
                    ArrowOfTrapdoor,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfTrapdoor>>,
    players: Query<&Transform, With<Player>>,
    doors: Query<&Transform, With<Trapdoor>>,
) {
    for player in &players {
        for door in &doors {
            if (player.translation.x - door.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
