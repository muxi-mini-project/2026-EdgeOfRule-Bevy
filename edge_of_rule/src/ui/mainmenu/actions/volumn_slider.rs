use bevy::prelude::*;

// 导入 spawner 中的组件
use crate::ui::mainmenu::spawner::volumn_slider::{SliderTrack, SliderHandle, SliderValueText, SliderValue};

// 单个函数处理所有拖动逻辑
pub fn on_drag(
    mut handle_query: Query<
        (Entity, &Interaction, &mut Style, &Parent),
        (With<SliderHandle>, Changed<Interaction>)
    >,
    mut background_query: Query<&mut BackgroundColor, With<SliderHandle>>,
    mut track_query: Query<&mut Style, (With<SliderTrack>, Without<SliderHandle>)>,
    mut text_query: Query<&mut Text, With<SliderValueText>>,
    mut value_query: Query<&mut SliderValue>,
    windows: Query<&Window>,
    mut is_dragging: Local<bool>,
    mut active_handle: Local<Option<Entity>>,  // 注意这里：存储的是 Entity，不是 Parent
) {
    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // ===== 第一部分：处理按下和释放 =====
    for (entity, interaction, mut handle_style, parent) in handle_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // 开始拖动 - 存储当前滑块的实体 ID
                *is_dragging = true;
                *active_handle = Some(entity);  // 直接存储 entity，不是 parent.get()
                
                // 通过 BackgroundColor 组件改变滑块颜色
                if let Ok(mut bg_color) = background_query.get_mut(entity) {
                    bg_color.0 = Color::rgb(0.7, 0.7, 0.8);
                }
            }
            Interaction::None => {
                // 如果这个滑块是被拖动的那个，结束拖动
                if *active_handle == Some(entity) {  // 比较 entity，不是 parent
                    *is_dragging = false;
                    *active_handle = None;
                    
                    // 恢复滑块颜色
                    if let Ok(mut bg_color) = background_query.get_mut(entity) {
                        bg_color.0 = Color::rgb(0.5, 0.5, 0.6);
                    }
                }
            }
            Interaction::Hovered => {
                // 悬停效果（只在没有拖动时生效）
                if *active_handle != Some(entity) {  // 比较 entity，不是 parent
                    if let Ok(mut bg_color) = background_query.get_mut(entity) {
                        bg_color.0 = Color::rgb(0.6, 0.6, 0.7);
                    }
                }
            }
        }
    }

    // ===== 第二部分：如果正在拖动，更新滑块位置 =====
    if *is_dragging {
        if let Some(active_entity) = *active_handle {  // active_entity 是 Entity 类型
            // 获取当前激活的滑块的样式
            if let Ok((_, _, mut handle_style, _)) = handle_query.get_mut(active_entity) {
                // 滑块的移动范围：left: 7.5% 到 77.5%
                let min_left = 7.5;
                let max_left = 77.5;
                
                // 将鼠标 X 坐标转换为百分比位置
                let mouse_x_percent = (cursor_position.x / window.width()) * 100.0;
                
                // 限制在有效范围内
                let new_left = mouse_x_percent.clamp(min_left, max_left);
                
                // 计算当前值 (0.0 - 1.0)
                let current_value = (new_left - min_left) / (max_left - min_left);
                
                // 更新滑块位置
                handle_style.left = Val::Percent(new_left);
                
                // 更新蓝色轨道的宽度
                for mut track_style in track_query.iter_mut() {
                    track_style.width = Val::Percent(70.0 * current_value);
                }
                
                // 更新百分比文本
                for mut text in text_query.iter_mut() {
                    let percentage = (current_value * 100.0).round() as i32;
                    text.sections[0].value = format!("{}%", percentage);
                }
                
                // 更新存储的值
                for mut value in value_query.iter_mut() {
                    value.0 = current_value;
                }
            }
        }
    }
}