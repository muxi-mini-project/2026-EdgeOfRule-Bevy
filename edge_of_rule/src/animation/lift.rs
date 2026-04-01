use bevy::prelude::*;

use crate::{entities::lift::Lift, levels::day2::scene3::actions::LiftState};

pub fn lift_animation_system(
    lift_state: Res<LiftState>,
    mut lifts: Query<&mut Handle<Image>, With<Lift>>,
    asset_server: Res<AssetServer>,
) {
    let next = match *lift_state {
        LiftState::Broken => asset_server.load("images/animations/lift/broken.png"),
        LiftState::Fixed => asset_server.load("images/animations/lift/fixed_0.png"),
    };

    for mut texture in &mut lifts {
        *texture = next.clone();
    }
}
