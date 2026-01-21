use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    assets::player::PlayerAssets,
    entities::player::{FacingDirection, Player, PlayerState},
};

pub fn player_animation_system(
    time: Res<Time>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut player: Query<(Entity, &Velocity, &Player, &mut Handle<Image>)>,
) {
    const VELOCITY_THRESHOLD: f32 = 10.0;

    for (entity, velocity, player, mut handle) in &mut player {
        let new_texture = match player.state {
            PlayerState::Dashing => Some(match player.facing {
                FacingDirection::Right => player_assets.side_squat_textures[0].clone(),
                FacingDirection::Left => player_assets.side_squat_textures[1].clone(),
            }),
            PlayerState::Sliding => Some(match player.facing {
                FacingDirection::Right => player_assets.slide_textures[0].clone(),
                FacingDirection::Left => player_assets.slide_textures[1].clone(),
            }),
            PlayerState::Crouching => Some(player_assets.front_squat_texture.clone()),
            PlayerState::Walking => {
                let step_out = ((time.elapsed_seconds() / 0.1) as i32) % 2 == 0;
                let facing_index = match (player.facing, step_out) {
                    (FacingDirection::Right, true) => 0,
                    (FacingDirection::Right, false) => 1,
                    (FacingDirection::Left, true) => 2,
                    (FacingDirection::Left, false) => 3,
                };
                Some(player_assets.side_textures[facing_index].clone())
            }
            PlayerState::Jumping | PlayerState::Falling | PlayerState::FastFalling => {
                if velocity.linvel.x.abs() > VELOCITY_THRESHOLD {
                    Some(match player.facing {
                        FacingDirection::Right => player_assets.side_textures[0].clone(),
                        FacingDirection::Left => player_assets.side_textures[2].clone(),
                    })
                } else {
                    Some(player_assets.front_texture.clone())
                }
            }
            PlayerState::Idle => Some(player_assets.front_texture.clone()),
        };

        if let Some(new_texture) = new_texture
            && *handle != new_texture
        {
            commands.entity(entity).insert(new_texture.clone());
            *handle = new_texture;
        }
    }
}
