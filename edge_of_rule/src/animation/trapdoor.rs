use bevy::prelude::*;

use crate::{entities::trapdoor::Trapdoor, levels::day1::scene2::TrapdoorState};

pub fn trapdoor_animation_system(
    mut trapdoors: Query<&mut Handle<Image>, With<Trapdoor>>,
    trapdoor_state: Res<TrapdoorState>,
    asset_server: Res<AssetServer>,
) {
    for mut texture in &mut trapdoors {
        *texture = match *trapdoor_state {
            TrapdoorState::Closed => asset_server.load("images/animations/trapdoor.png"),
            TrapdoorState::Opened => {
                asset_server.load("images/levels/day1/scene2_opened_trapdoor.png")
            }
        }
    }
}
