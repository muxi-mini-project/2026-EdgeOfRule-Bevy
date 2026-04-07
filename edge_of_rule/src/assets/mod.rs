pub mod arrow;
pub mod door;
pub mod elevator;
pub mod hud;
pub mod levels;
pub mod lift;
pub mod player;
pub mod ui_image;

use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player::load_player_assets)
            .add_systems(Startup, levels::load_levels_image_assets)
            .add_systems(Startup, door::load_door_assets)
            .add_systems(Startup, elevator::load_elevator_assets)
            .add_systems(Startup, lift::load_lift_assets)
            .add_systems(Startup, arrow::load_arrow_assets)
            .add_systems(Startup, hud::load_hud_image_assets);
    }
}
