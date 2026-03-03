use bevy::prelude::*;

use crate::entities::{arrow::spawn_arrow, player::Player, small_note::SmallNote};

#[derive(Component)]
pub struct ArrowOfSmallNote;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    arrows: Query<&ArrowOfSmallNote>,
    players: Query<&Transform, With<Player>>,
    notes: Query<&Transform, With<SmallNote>>,
) {
    if arrows.iter().len() != 0 {
        return;
    }

    for player in &players {
        for note in &notes {
            if (player.translation.x - note.translation.x).abs() < 60.0 {
                spawn_arrow(
                    &mut commands,
                    Transform::from_xyz(8.0, 42.0, -4.0),
                    &asset_server,
                    ArrowOfSmallNote,
                );
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    arrows: Query<Entity, With<ArrowOfSmallNote>>,
    players: Query<&Transform, With<Player>>,
    notes: Query<&Transform, With<SmallNote>>,
) {
    for player in &players {
        for note in &notes {
            if (player.translation.x - note.translation.x).abs() < 60.0 {
                return;
            }
        }
    }

    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}

pub fn despawn_all(mut commands: Commands, arrows: Query<Entity, With<ArrowOfSmallNote>>) {
    for arrow in &arrows {
        commands.entity(arrow).despawn();
    }
}
