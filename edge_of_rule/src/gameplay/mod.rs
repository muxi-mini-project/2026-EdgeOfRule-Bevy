use bevy::prelude::*;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<State>();
    }
}

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum State {
    #[default]
    MainMenu,
    InGame,
}
