use bevy::prelude::*;

use crate::{
    assets::door::DoorAssets, entities::trapdoor::Trapdoor, levels::day1::scene2::TrapdoorState,
};

pub fn trapdoor_animation_system(
    mut trapdoors: Query<&mut Handle<Image>, With<Trapdoor>>,
    trapdoor_state: Res<TrapdoorState>,
    asset_server: Res<DoorAssets>,
) {
    for mut texture in &mut trapdoors {
        *texture = match *trapdoor_state {
            TrapdoorState::Closed => asset_server.trapdoor.clone(),
            TrapdoorState::Opened => asset_server.trapdoor_opened.clone(),
        }
    }
}
