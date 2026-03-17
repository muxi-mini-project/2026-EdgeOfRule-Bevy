use bevy::prelude::*;

use crate::ui::mainmenu::spawner::{arrow_rules::ArrowRules, rules_btn::RulesBtn};

pub fn on_hover(
    btns: Query<&Interaction, With<RulesBtn>>,
    mut query: Query<&mut Visibility, With<ArrowRules>>,
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