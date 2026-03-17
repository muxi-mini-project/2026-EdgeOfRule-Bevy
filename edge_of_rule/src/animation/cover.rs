use bevy::prelude::*;

use crate::{entities::cover::Cover, levels::day1::scene3::Scene3CoverState};

pub fn cover_animation_system(
    mut chests: Query<&mut Handle<Image>, With<Cover>>,
    chest_state: Res<Scene3CoverState>,
    asset_server: Res<AssetServer>,
) {
    for mut texture in &mut chests {
        *texture = match *chest_state {
            Scene3CoverState::Packed => asset_server.load("images/animations/cover.png"),
            Scene3CoverState::Opened => asset_server.load("images/animations/key.png"),
            Scene3CoverState::Picked => asset_server.load("images/animations/none.png"),
        }
    }
}
