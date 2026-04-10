use bevy::prelude::*;

use crate::{
    animation::lift_door::LiftDoorAnim, assets::lift::LiftAssets, entities::lift::Lift,
    levels::day2::scene3::actions::LiftState,
};

pub fn lift_animation_system(
    lift_state: Res<LiftState>,
    lift_door: Res<LiftDoorAnim>,
    mut lifts: Query<&mut Handle<Image>, With<Lift>>,
    assets: Res<LiftAssets>,
) {
    let next = if lift_door.active {
        match lift_door.frame {
            0 => assets.fixed_1.clone(),
            1 => assets.fixed_2.clone(),
            _ => assets.fixed_3.clone(),
        }
    } else {
        match *lift_state {
            LiftState::Broken => assets.broken.clone(),
            LiftState::Fixed => assets.fixed_0.clone(),
        }
    };

    for mut texture in &mut lifts {
        *texture = next.clone();
    }
}
