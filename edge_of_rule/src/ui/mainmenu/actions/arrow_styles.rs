use bevy::prelude::*;

use crate::ui::mainmenu::spawner::{arrow_styles::ArrowStyles, style_select_btn::StyleSelectBtn};

pub fn on_hover(
    btns: Query<&Interaction, With<StyleSelectBtn>>,
    mut query: Query<&mut Visibility, With<ArrowStyles>>,
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