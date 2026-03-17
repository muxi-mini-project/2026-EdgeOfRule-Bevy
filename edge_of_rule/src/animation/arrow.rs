use bevy::prelude::*;

use crate::{assets::arrow::ArrowAssets, entities::arrow::Arrow};

pub fn arrow_animation_system(
    time: Res<Time>,
    mut arrows: Query<&mut Handle<Image>, With<Arrow>>,
    asset_server: Res<ArrowAssets>,
) {
    for mut texture in &mut arrows {
        *texture = match ((time.elapsed_seconds() / 0.1) as i32) % 3 {
            0 => asset_server.up.clone(),
            1 => asset_server.mid.clone(),
            2 => asset_server.down.clone(),
            _ => unreachable!(),
        }
    }
}
