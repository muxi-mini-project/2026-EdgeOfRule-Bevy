pub mod levels;
pub mod player;
pub mod ui_image;

use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player::load_player_assets)
            .add_systems(Startup, levels::load_levels_image_assets);
    }
}
