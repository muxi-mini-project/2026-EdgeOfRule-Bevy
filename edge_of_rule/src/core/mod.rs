pub mod health;
pub mod state;

use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<state::GameState>()
            .init_resource::<health::PlayerHealth>()
            .init_resource::<health::SewageDamageAccum>()
            .add_event::<health::PlayerDied>();
    }
}
