use bevy::prelude::*;

use crate::{
    assets::elevator::ElevatorAssets,
    entities::elevator::{Elevator, ElevatorAnimState, ElevatorSprite},
};

pub fn elevator_animation_system(
    asset_server: Res<ElevatorAssets>,
    mut elevators: Query<(&Transform, &mut ElevatorAnimState, &Children), With<Elevator>>,
    mut sprites: Query<&mut Handle<Image>, With<ElevatorSprite>>,
) {
    const EPS: f32 = 0.05;

    for (transform, mut anim_state, children) in &mut elevators {
        let dy = transform.translation.y - anim_state.last_y;
        anim_state.last_y = transform.translation.y;

        let next = if dy > EPS {
            asset_server.up.clone()
        } else if dy < -EPS {
            asset_server.down.clone()
        } else {
            asset_server.stop.clone()
        };

        for &child in children.iter() {
            if let Ok(mut handle) = sprites.get_mut(child) {
                *handle = next.clone();
            }
        }
    }
}
