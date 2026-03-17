use bevy::prelude::*;

use crate::ui::mainmenu::spawner::{arrow_levels::ArrowLevels, game_start_btn::GameStartBtn};

pub fn on_hover(
    btns: Query<&Interaction, With<GameStartBtn>>,
    mut query: Query<&mut Visibility, With<ArrowLevels>>,
) {
    let any_hovered = btns.iter().any(|&interaction| interaction == Interaction::Hovered);
    
    for mut visibility in &mut query {
        *visibility = if any_hovered {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}