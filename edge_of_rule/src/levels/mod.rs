pub mod day1;
pub mod day2;

use bevy::prelude::*;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(day1::scene1::Scene1Plugin)
            .add_plugins(day1::scene2::Scene2Plugin)
            .add_plugins(day1::scene3::Scene3Plugin)
            .add_plugins(day1::scene4::Scene4Plugin)
            .add_plugins(day2::scene1::Scene1Plugin)
            .add_plugins(day2::scene3::Scene3Plugin)
            .add_plugins(day2::scene4::Scene4Plugin);
    }
}
