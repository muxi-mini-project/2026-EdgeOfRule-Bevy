use bevy::prelude::*;

use crate::{entities::chest::Chest, levels::day1::scene3::Scene3ChestState};

pub fn chest_animation_system(
    mut chests: Query<&mut Handle<Image>, With<Chest>>,
    chest_state: Res<Scene3ChestState>,
    asset_server: Res<AssetServer>,
) {
    for mut texture in &mut chests {
        *texture = match *chest_state {
            Scene3ChestState::Packed => asset_server.load("images/animations/chest.png"),
            Scene3ChestState::Opened => asset_server.load("images/animations/controller.png"),
            Scene3ChestState::Picked => asset_server.load("images/animations/none.png"),
        }
    }
}
