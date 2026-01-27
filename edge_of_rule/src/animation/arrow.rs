use bevy::prelude::*;

use crate::entities::arrow::Arrow;

pub fn arrow_animation_system(
    time: Res<Time>,
    mut arrows: Query<&mut Handle<Image>, With<Arrow>>,
    asset_server: Res<AssetServer>,
) {
    for mut texture in &mut arrows {
        *texture = match ((time.elapsed_seconds() / 0.1) as i32) % 3 {
            0 => asset_server.load("images/HUD/arrow/0.png"),
            1 => asset_server.load("images/HUD/arrow/1.png"),
            2 => asset_server.load("images/HUD/arrow/2.png"),
            _ => unreachable!(),
        }
    }
}
