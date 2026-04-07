use bevy::prelude::*;

use crate::{
    assets::lift::LiftAssets,
    entities::lift::Lift,
    levels::day2::scene3::actions::{LiftOpenAnim, LiftState},
};

pub fn lift_animation_system(
    lift_state: Res<LiftState>,
    lift_open: Res<LiftOpenAnim>,
    mut lifts: Query<&mut Handle<Image>, With<Lift>>,
    assets: Res<LiftAssets>,
) {
    let next = if lift_open.active {
        match lift_open.frame {
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
