use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::player::PlayerAssets, entities::player::Player};

pub fn player_walk(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut player: Query<(Entity, &Velocity, &Player, &mut Handle<Image>)>,
) {
    const VELOCITY_THERESHOULD: f32 = 10.0;

    for (entity, velocity, player, mut handle) in &mut player {
        let vx = velocity.linvel.x;
        let step_out = ((time.elapsed_seconds() / 0.1) as i32) % 2 == 0;
        let horizontal_pressed = keyboard_input.pressed(KeyCode::KeyA)
            || keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::KeyD)
            || keyboard_input.pressed(KeyCode::ArrowRight);
        let squat_pressed =
            keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
        let shift_pressed = keyboard_input.pressed(KeyCode::ShiftLeft)
            || keyboard_input.pressed(KeyCode::ShiftRight);
        let sliding_input = squat_pressed && horizontal_pressed && !shift_pressed;
        let show_dash_texture =
            player.is_dashing && (shift_pressed || player.dash_direction.y != 0.0);

        let new_texture = if show_dash_texture {
            Some(if vx >= 0.0 {
                player_assets.side_squat_textures[0].clone()
            } else {
                player_assets.side_squat_textures[1].clone()
            })
        } else if squat_pressed && !sliding_input {
            Some(player_assets.front_squat_texture.clone())
        } else if vx.abs() > VELOCITY_THERESHOULD {
            Some(match (vx, step_out) {
                (vx, true) if vx > 0.0 => player_assets.side_textures[0].clone(),
                (vx, false) if vx > 0.0 => player_assets.side_textures[1].clone(),
                (_, true) => player_assets.side_textures[2].clone(),
                (_, false) => player_assets.side_textures[3].clone(),
            })
        } else {
            Some(player_assets.front_texture.clone())
        };

        if let Some(new_texture) = new_texture {
            if *handle != new_texture {
                commands.entity(entity).insert(new_texture.clone());
                *handle = new_texture;
            }
        }
    }
}
