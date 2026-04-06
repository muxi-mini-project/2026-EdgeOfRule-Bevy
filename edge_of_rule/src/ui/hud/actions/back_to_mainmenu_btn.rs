use bevy::prelude::*;
use crate::core::state::GameState;
use crate::ui::hud::spawner::ingame_option_area::{  
    InGameOptionArea, 
    InGameOptionTitle, 
    InGameExitBtn, 
    UnderTip, 
    KeysTip,
    KeysWords,
    BackToMainMenuBtn,
};

pub fn on_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackToMainMenuBtn>)>,
    mut state: ResMut<NextState<GameState>>,
    mut param_set: ParamSet<(
        Query<&mut Visibility, With<InGameOptionArea>>,  
        Query<&mut Visibility, With<InGameOptionTitle>>, 
        Query<&mut Visibility, With<InGameExitBtn>>,     
        Query<&mut Visibility, With<UnderTip>>, 
        Query<&mut Visibility, With<BackToMainMenuBtn>>,  
        Query<&mut Visibility, With<KeysTip>>,    
        Query<&mut Visibility, With<KeysWords>>,   
    )>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            state.set(GameState::MainMenu);
            for mut visibility in param_set.p0().iter_mut() {
                *visibility = Visibility::Hidden;
            }
            for mut visibility in param_set.p1().iter_mut() {
                *visibility = Visibility::Hidden;
            }
            for mut visibility in param_set.p2().iter_mut() {
                *visibility = Visibility::Hidden;
            }
            for mut visibility in param_set.p3().iter_mut() {
                *visibility = Visibility::Hidden;
            }
            for mut visibility in param_set.p4().iter_mut() {
                *visibility = Visibility::Hidden;
            }
            for mut visibility in param_set.p5().iter_mut() {
                *visibility = Visibility::Hidden;
            }
            for mut visibility in param_set.p6().iter_mut() {
                *visibility = Visibility::Hidden;
            }

        }
    }
}   

