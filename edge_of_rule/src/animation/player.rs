use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::player::PlayerAssets, entities::player::Player};

pub fn player_walk(
    time: Res<Time>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut player: Query<(Entity, &Velocity, &mut Handle<Image>), With<Player>>,
) {
    const VELOCITY_THERESHOULD: f32 = 10.0;

    for (entity, velocity, mut handle) in &mut player {
        let vx = velocity.linvel.x;
        let step_out = ((time.elapsed_seconds() / 0.1) as i32) % 2 == 0;

        if vx.abs() > VELOCITY_THERESHOULD {
            let new_texture = match (vx, step_out) {
                (vx, true) if vx > 0.0 => player_assets.side_textures[0].clone(),
                (vx, false) if vx > 0.0 => player_assets.side_textures[1].clone(),
                (_, true) => player_assets.side_textures[2].clone(),
                (_, false) => player_assets.side_textures[3].clone(),
            };

            if *handle != new_texture {
                commands.entity(entity).insert(new_texture.clone());
                *handle = new_texture;
            }
        } else {
            let new_texture = player_assets.front_texture.clone();

            if *handle != new_texture {
                commands.entity(entity).insert(new_texture.clone());
                *handle = new_texture;
            }
        }
    }
}
