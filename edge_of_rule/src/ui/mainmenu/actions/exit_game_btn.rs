use bevy::prelude::*;

use crate::{core::state::GameState, ui::mainmenu::spawner::exit_game_btn::ExitGameBtn};

pub fn on_click(
    mut exit_game: EventWriter<bevy::app::AppExit>,
    mut btns: Query<(&Interaction,&mut BackgroundColor), With<ExitGameBtn>>,
) {
    for (reaction,mut color) in &mut btns {
        match *reaction {
            Interaction::Pressed => {
                *color = BackgroundColor::from(Color::rgba(65.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0, 1.0));
                exit_game.send(bevy::app::AppExit);
            }
            Interaction::Hovered => {
                *color = BackgroundColor::from(Color::rgb(115.0 / 255.0, 7.0 / 255.0, 7.0 / 255.0));
            }
            Interaction::None => {
                *color = BackgroundColor::from(Color::rgba(82.0 / 255.0, 4.0 / 255.0, 4.0 / 255.0, 1.0));
            }
        }
    }
}
