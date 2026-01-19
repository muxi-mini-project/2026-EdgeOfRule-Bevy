mod player;

use bevy::prelude::*;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<State>()
            .add_systems(Startup, player::load_player_assets)
            .add_systems(OnEnter(State::InGame), player::spawn_player);
    }
}

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum State {
    #[default]
    MainMenu,
    InGame,
}
