pub mod general;
pub mod player;

use crate::core::state::GameState;
use bevy::prelude::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), player::spawn_player);
    }
}
