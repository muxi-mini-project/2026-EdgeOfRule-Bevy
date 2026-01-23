pub mod day1;

use bevy::prelude::*;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(day1::scene1::Scene1Plugin)
            .add_plugins(day1::scene2::Scene2Plugin);
    }
}
