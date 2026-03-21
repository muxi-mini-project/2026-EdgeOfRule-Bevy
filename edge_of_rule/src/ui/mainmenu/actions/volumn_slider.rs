use bevy::prelude::*;

// 导入 spawner 中的组件
use crate::ui::mainmenu::spawner::volumn_slider::{SliderTrack, SliderHandle, SliderValueText, SliderValue};

// 添加一个资源来跟踪拖动状态
#[derive(Resource)]
pub struct DraggingState {
    pub(crate) handle_entity: Entity,
    pub(crate) min_left: f32,
    pub(crate) max_left: f32,
}

pub fn on_drag(
    mut commands: Commands,
    interaction_query: Query<
        (Entity, &Interaction),
        (With<SliderHandle>, Changed<Interaction>)
    >,
    mut handle_query: Query<&mut Style, With<SliderHandle>>,
    mut background_query: Query<&mut BackgroundColor, With<SliderHandle>>,
    mut track_query: Query<&mut Style, (With<SliderTrack>, Without<SliderHandle>)>,
    mut text_query: Query<&mut Text, With<SliderValueText>>,
    mut value_query: Query<&mut SliderValue>,
    windows: Query<&Window>,
    mut dragging_state: Option<ResMut<DraggingState>>,
) {
    let window = windows.single();
    
    // ===== 处理交互变化 =====
    for (entity, interaction) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                if dragging_state.is_none() {
                    let min_left = 7.5;
                    let max_left = 77.5;
                    
                    commands.insert_resource(DraggingState {
                        handle_entity: entity,
                        min_left,
                        max_left,
                    });
                    
                    if let Ok(mut bg_color) = background_query.get_mut(entity) {
                        bg_color.0 = Color::rgb(0.7, 0.7, 0.8);
                    }
                }
            }
            Interaction::None => {
                if let Some(dragging) = dragging_state.as_ref() {
                    if dragging.handle_entity == entity {
                        commands.remove_resource::<DraggingState>();
                        
                        if let Ok(mut bg_color) = background_query.get_mut(entity) {
                            bg_color.0 = Color::rgb(0.5, 0.5, 0.6);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                if dragging_state.is_none() {
                    if let Ok(mut bg_color) = background_query.get_mut(entity) {
                        bg_color.0 = Color::rgb(0.6, 0.6, 0.7);
                    }
                }
            }
        }
    }
    
    // ===== 更新拖动位置 =====
    if let Some(dragging) = dragging_state.as_mut() {
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        
        if let Ok(mut handle_style) = handle_query.get_mut(dragging.handle_entity) {
            let mouse_x_percent = (cursor_position.x / window.width()) * 100.0;
            let new_left = mouse_x_percent.clamp(dragging.min_left, dragging.max_left);
            let current_value = (new_left - dragging.min_left) / (dragging.max_left - dragging.min_left);
            
            handle_style.left = Val::Percent(new_left);
            
            for mut track_style in track_query.iter_mut() {
                track_style.width = Val::Percent(70.0 * current_value);
            }
            
            for mut text in text_query.iter_mut() {
                let percentage = (current_value * 100.0).round() as i32;
                text.sections[0].value = format!("{}%", percentage);
            }
            
            for mut value in value_query.iter_mut() {
                value.0 = current_value;
            }
        } else {
            commands.remove_resource::<DraggingState>();
        }
    }
    
    // 恢复非悬停状态的颜色（如果没有在拖动）
    if dragging_state.is_none() {
        for (entity, interaction) in interaction_query.iter() {
            if *interaction != Interaction::Hovered {
                if let Ok(mut bg_color) = background_query.get_mut(entity) {
                    bg_color.0 = Color::rgb(0.5, 0.5, 0.6);
                }
            }
        }
    }
}