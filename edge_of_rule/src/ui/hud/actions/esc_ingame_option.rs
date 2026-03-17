use bevy::prelude::*;

use crate::ui::hud::spawner::ingame_option_area::{InGameOptionArea, InGameExitBtn, InGameOptionTitle, UnderTip};

#[derive(Resource, Default)]
pub struct OptionSpawnState {
    pub is_visible: bool,
}

pub fn on_key_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<OptionSpawnState>,
    mut param_set: ParamSet<(
        Query<&mut Visibility, With<InGameOptionArea>>,  // 根节点
        Query<&mut Visibility, With<InGameOptionTitle>>, // 标题
        Query<&mut Visibility, With<InGameExitBtn>>,     // 按钮
        Query<&mut Visibility, With<UnderTip>>,          // 提示
    )>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        menu_state.is_visible = !menu_state.is_visible;
        
        let new_visibility = if menu_state.is_visible {
            info!("显示菜单");
            Visibility::Visible
        } else {
            info!("隐藏菜单");
            Visibility::Hidden
        };
        
        // 依次使用每个查询，ParamSet确保它们不会同时被借用
        for mut visibility in param_set.p0().iter_mut() {
            *visibility = new_visibility;
        }
        
        for mut visibility in param_set.p1().iter_mut() {
            *visibility = new_visibility;
        }
        
        for mut visibility in param_set.p2().iter_mut() {
            *visibility = new_visibility;
        }
        
        for mut visibility in param_set.p3().iter_mut() {
            *visibility = new_visibility;
        }
    }
}

pub fn on_click(
    mut exit_game: EventWriter<bevy::app::AppExit>,
    mut btns: Query<(&Interaction,&mut BackgroundColor), With<InGameExitBtn>>,
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