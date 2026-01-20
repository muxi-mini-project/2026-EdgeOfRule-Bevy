use bevy::prelude::*;
use crate::core::state::GameState::MainMenu;

pub struct UiPlugin;

// ==================== 组件定义 ====================

#[derive(Component)]
struct BtnOptions;

#[derive(Component)]
struct BtnSelStyle;

#[derive(Component)]
struct BtnStartGame;

#[derive(Component)]
struct BtnRules;

#[derive(Component)]
struct ClosePopupButton;

#[derive(Component)]
struct QuitGameButton;

// 弹窗组件
#[derive(Component)]
struct Popup;

#[derive(Component)]
struct SettingsPopup;

#[derive(Component)]
struct StyleSelectPopup;

#[derive(Component)]
struct RulesPopup;

// 音量控制相关组件
#[derive(Component)]
struct VolumeSlider;

#[derive(Component)]
struct VolumeSliderHandle;

#[derive(Component)]
struct VolumeSliderTrack;

#[derive(Component)]
struct VolumeValueText;

// 添加一个组件来阻止事件传递
#[derive(Component)]
struct BlockEvents;

// 添加组件跟踪滑块是否被拖拽
#[derive(Component)]
struct DraggingSlider(bool);

// ==================== 资源定义 ====================

#[derive(Resource)]
struct BackgroundImageHandle(Handle<Image>);

#[derive(Resource)]
struct OptionsImageHandle(Handle<Image>);

#[derive(Resource)]
struct MainFontHandle(Handle<Font>);

#[derive(Resource, Default)]
struct PopupState {
    active_popup: Option<PopupType>,
}

// 添加音量资源
#[derive(Resource, Default)]
struct VolumeSettings {
    volume: f32, // 0.0 到 1.0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PopupType {
    Settings,
    StyleSelect,
    Rules,
}

// 添加一个标记组件，表示主菜单UI是否已创建
#[derive(Component)]
struct MainMenuUi;

// 添加一个资源来跟踪UI是否已初始化
#[derive(Resource, Default)]
struct UiInitialized(bool);

// ==================== 资源加载 ====================

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BackgroundImageHandle(
        asset_server.load("images/background.png")
    ));
    commands.insert_resource(OptionsImageHandle(
        asset_server.load("images/options.png")
    ));
    commands.insert_resource(MainFontHandle(
        asset_server.load("images/font/aLiFont.ttf")
    ));
    commands.insert_resource(PopupState::default());
    commands.insert_resource(VolumeSettings { volume: 1.0 });
    commands.insert_resource(UiInitialized(false));
}

// ==================== 主菜单UI创建 ====================

fn setup_main_menu(
    mut commands: Commands,
    background_image: Option<Res<BackgroundImageHandle>>,
    options_image: Option<Res<OptionsImageHandle>>,
    font_handle: Option<Res<MainFontHandle>>,
    mut ui_initialized: ResMut<UiInitialized>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    // 如果UI已经初始化，直接返回
    if ui_initialized.0 {
        return;
    }
    
    // 检查所有资源是否都已加载
    if background_image.is_none() || options_image.is_none() || font_handle.is_none() {
        return;
    }
    
    let background_image = background_image.unwrap();
    let options_image = options_image.unwrap();
    let font_handle = font_handle.unwrap();
    
    // 确保有一个相机
    if camera_query.is_empty() {
        commands.spawn(Camera2dBundle::default());
    }
    
    // 主容器
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            ..default()
        },
        Name::new("MainMenu"),
        MainMenuUi,
    ))
    .with_children(|parent| {
        // 背景图片
        parent.spawn(
            ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: UiImage::new(background_image.0.clone()),
                ..default()
            },
        );
        
        // 设置按钮（右上角）
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(4.73),
                    height: Val::Percent(7.86),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(2.0),
                    left: Val::Percent(1.89),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            },
            BtnOptions,
        ))
        .with_children(|parent| {
            parent.spawn(
                ImageBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    image: UiImage::new(options_image.0.clone()),
                    ..default()
                },
            );
        });
        
        // 按钮容器（居中）
        parent.spawn(
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(42.45),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            },
        )
        .with_children(|parent| {
            // Select Style 按钮（带渐变文字）
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(25.0),
                        height: Val::Percent(9.52),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Percent(0.0),
                            bottom: Val::Percent(6.39),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0x53 as f32 / 255.0, 0x8d as f32 / 255.0, 0xd2 as f32 / 255.0, 1.0)),
                    ..default()
                },
                BtnSelStyle,
            ))
            .with_children(|parent| {
                // 创建渐变文字效果
                create_gradient_text(parent, "Select Style", font_handle.0.clone());
            });
            
            // Game Start 按钮（带渐变文字）
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(19.8),
                        height: Val::Percent(9.52),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Percent(0.0),
                            bottom: Val::Percent(6.39),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0x53 as f32 / 255.0, 0x8d as f32 / 255.0, 0xd2 as f32 / 255.0, 1.0)),
                    ..default()
                },
                BtnStartGame,
            ))
            .with_children(|parent| {
                create_gradient_text(parent, "Game Start", font_handle.0.clone());
            });
            
            // The Rules 按钮（带渐变文字）
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(19.8),
                        height: Val::Percent(9.52),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Percent(0.0),
                            bottom: Val::Percent(0.0),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0x53 as f32 / 255.0, 0x8d as f32 / 255.0, 0xd2 as f32 / 255.0, 1.0)),
                    ..default()
                },
                BtnRules,
            ))
            .with_children(|parent| {
                create_gradient_text(parent, "the Rules", font_handle.0.clone());
            });
        });
    });
    
    // 标记UI已初始化
    ui_initialized.0 = true;
}

// ==================== 创建渐变文字的函数 ====================

fn create_gradient_text(parent: &mut ChildBuilder, text: &str, font: Handle<Font>) {
    let chars: Vec<char> = text.chars().collect();
    let total_chars = chars.len() as f32;
    
    for (i, ch) in chars.iter().enumerate() {
        let position = if total_chars > 1.0 {
            i as f32 / (total_chars - 1.0)
        } else {
            0.5
        };
        let r = (114.0 + position * 44.0) / 255.0;
        let g = (149.0 + position * 14.0) / 255.0;
        let b = (176.0 + position * 9.0) / 255.0;
        
        parent.spawn(TextBundle::from_section(
            ch.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 60.0,
                color: Color::rgb(r, g, b),
                ..default()
            },
        ));
    }
}

// ==================== 弹窗创建函数 ====================

fn create_settings_popup(
    parent: &mut ChildBuilder, 
    font: Handle<Font>,
    volume_settings: &VolumeSettings,
) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(60.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.3)),
            ..default()
        },
        SettingsPopup,
    ))
    .with_children(|popup| {
        // 标题
        popup.spawn(TextBundle::from_section(
            "Settings",
            TextStyle {
                font: font.clone(),
                font_size: 50.0,
                color: Color::WHITE,
                ..default()
            },
        ));
        
        // 设置选项
        popup.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.0)),
                row_gap: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .with_children(|settings| {
            // 音量控制
            settings.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.4)),
                ..default()
            })
            .with_children(|volume_row| {
                volume_row.spawn(TextBundle::from_section(
                    "Volume",
                    TextStyle {
                        font: font.clone(),
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                
                // 音量数值显示
                volume_row.spawn((
                    TextBundle::from_section(
                        format!("{:.0}%", volume_settings.volume * 100.0),
                        TextStyle {
                            font: font.clone(),
                            font_size: 25.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    VolumeValueText,
                ));
            });
            
            // 音量滑块容器
            settings.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::horizontal(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                },
                Name::new("VolumeSliderContainer"),
            ))
            .with_children(|slider_container| {
                // 滑块轨道背景
                slider_container.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(90.0),
                            height: Val::Px(8.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    VolumeSliderTrack,
                ));
                
                // 滑块填充（表示当前音量）
                let fill_width = volume_settings.volume * 90.0;
                slider_container.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(fill_width),
                            height: Val::Px(8.0),
                            position_type: PositionType::Absolute,
                            left: Val::Percent(5.0), // 左边距5%
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.2, 0.6, 0.9)),
                        ..default()
                    },
                    VolumeSlider,
                ));
                
                // 滑块手柄
                slider_container.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(24.0),
                            height: Val::Px(24.0),
                            position_type: PositionType::Absolute,
                            left: Val::Percent(volume_settings.volume * 90.0 + 5.0 - 1.2), // 计算手柄位置
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.9, 0.9, 0.9)),
                        ..default()
                    },
                    VolumeSliderHandle,
                    DraggingSlider(false),
                ));
            });
            
            // 显示模式示例
            settings.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.4)),
                ..default()
            })
            .with_children(|display_row| {
                display_row.spawn(TextBundle::from_section(
                    "Fullscreen",
                    TextStyle {
                        font: font.clone(),
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                
                display_row.spawn(TextBundle::from_section(
                    "Off",
                    TextStyle {
                        font: font.clone(),
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
        
        // 按钮容器
        popup.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(30.0)),
                row_gap: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .with_children(|buttons| {
            // 退出游戏按钮
            buttons.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        padding: UiRect::all(Val::Px(15.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb(156.0/255.0, 4.0/255.0, 4.0/255.0)),
                    ..default()
                },
                QuitGameButton,
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "Quit Game",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
            
            // 关闭按钮
            buttons.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        padding: UiRect::all(Val::Px(15.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.2, 0.5, 0.8)),
                    ..default()
                },
                ClosePopupButton,
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "Close",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
    });
}

fn create_style_select_popup(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(70.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.3, 0.2, 0.3)),
            ..default()
        },
        StyleSelectPopup,
    ))
    .with_children(|popup| {
        popup.spawn(TextBundle::from_section(
            "Select Game Style",
            TextStyle {
                font: font.clone(),
                font_size: 45.0,
                color: Color::WHITE,
                ..default()
            },
        ));
        
        // 风格选项
        popup.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(30.0)),
                row_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        })
        .with_children(|styles| {
            let style_options = ["easy", "normal", "hard"];
            
            for style in style_options {
                styles.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.4, 0.4, 0.6)),
                        ..default()
                    },
                    Name::new(format!("StyleBtn_{}", style)),
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        style,
                        TextStyle {
                            font: font.clone(),
                            font_size: 35.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            }
        });
        
        // 关闭按钮
        popup.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(30.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.8, 0.2, 0.2)),
                ..default()
            },
            ClosePopupButton,
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "Close",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

fn create_rules_popup(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(85.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.3, 0.2)),
            ..default()
        },
        RulesPopup,
    ))
    .with_children(|popup| {
        popup.spawn(TextBundle::from_section(
            "Game Rules",
            TextStyle {
                font: font.clone(),
                font_size: 50.0,
                color: Color::WHITE,
                ..default()
            },
        ));
        
        // 规则内容
        popup.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                margin: UiRect::top(Val::Px(20.0)),
                row_gap: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .with_children(|rules| {
            let rule_items = [
                "1. Use arrow keys to move",
                "2. Collect all coins to win",
                "3. Avoid enemies",
                "4. You have 3 lives",
                "5. Complete within time limit",
            ];
            
            for rule in rule_items {
                rules.spawn(TextBundle::from_section(
                    rule,
                    TextStyle {
                        font: font.clone(),
                        font_size: 28.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            }
        });
        
        // 关闭按钮
        popup.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(30.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.8, 0.2, 0.2)),
                ..default()
            },
            ClosePopupButton,
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "Close",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

// ==================== 音量控制函数 ====================

// 开始拖拽滑块
fn start_dragging_slider(
    mut handle_query: Query<(&GlobalTransform, &mut DraggingSlider), With<VolumeSliderHandle>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_query.single();
        
        if let Some(cursor_position) = window.cursor_position() {
            // 转换鼠标屏幕坐标为世界坐标
            if let Some(_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                // 检查鼠标是否在滑块手柄上
                for (handle_transform, mut dragging) in handle_query.iter_mut() {
                    let handle_world_pos = handle_transform.translation();
                    
                    // 滑块手柄大小约为40x40像素（根据创建时的宽度和高度）
                    // 转换到百分比：40/1280 ≈ 3.125%
                    let handle_half_width = 1.5; // 百分比单位
                    let handle_half_height = 2.0; // 百分比单位
                    
                    // 计算鼠标相对于窗口的百分比位置
                    let mouse_percent_x = cursor_position.x / window.width() * 100.0;
                    let mouse_percent_y = cursor_position.y / window.height() * 100.0;
                    
                    // 获取滑块手柄的位置（百分比）
                    let handle_x = (handle_world_pos.x / window.width() * 100.0) + 50.0; // 转换到百分比
                    let handle_y = (handle_world_pos.y / window.height() * 100.0) + 50.0;
                    
                    // 检查鼠标是否在滑块手柄范围内
                    if (mouse_percent_x - handle_x).abs() < handle_half_width 
                        && (mouse_percent_y - handle_y).abs() < handle_half_height {
                        dragging.0 = true;
                    }
                }
            }
        }
    }
}

// 停止拖拽滑块
fn stop_dragging_slider(
    mut handle_query: Query<&mut DraggingSlider, With<VolumeSliderHandle>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        for mut dragging in handle_query.iter_mut() {
            dragging.0 = false;
        }
    }
}

// 更新音量滑块位置 - 使用分开的查询避免冲突
fn update_volume_slider(
    mut volume_settings: ResMut<VolumeSettings>,
    mut handle_query: Query<(&mut Style, &DraggingSlider), With<VolumeSliderHandle>>,
    mut slider_query: Query<&mut Style, (With<VolumeSlider>, Without<VolumeSliderHandle>)>,
    mut volume_text_query: Query<&mut Text, With<VolumeValueText>>,
    windows: Query<&Window>,
) {
    // 检查是否有拖拽中的滑块
    for (mut handle_style, dragging) in handle_query.iter_mut() {
        // 只有在dragging.0为true时才更新位置
        if dragging.0 {
            // 获取鼠标位置
            let window = windows.single();
            
            if let Some(cursor_position) = window.cursor_position() {
                // 获取窗口尺寸
                let window_width = window.width();
                
                // 计算鼠标在窗口中的百分比位置
                let mouse_percent = cursor_position.x / window_width;
                
                // 滑块有效范围：5% 到 95%（因为滑块容器有5%的左右边距）
                let min_percent = 0.05;
                let max_percent = 0.95;
                
                // 限制鼠标位置在滑块范围内
                let clamped_percent = mouse_percent.clamp(min_percent, max_percent);
                
                // 将范围映射到0-1的音量值
                let volume = (clamped_percent - min_percent) / (max_percent - min_percent);
                
                // 更新音量
                volume_settings.volume = volume;
                
                // 更新滑块手柄位置
                let slider_position = volume * 90.0 + 5.0 - 1.2; // 计算百分比位置
                handle_style.left = Val::Percent(slider_position);
                
                // 更新滑块填充宽度
                for mut slider_style in slider_query.iter_mut() {
                    slider_style.width = Val::Percent(volume * 90.0);
                }
                
                // 更新音量文本
                for mut text in volume_text_query.iter_mut() {
                    text.sections[0].value = format!("{:.0}%", volume * 100.0);
                }
            }
        }
    }
}

// 更新音量到系统（如果需要应用到音频）
fn update_volume_system(volume_settings: Res<VolumeSettings>) {
    if volume_settings.is_changed() {
        println!("Volume changed to: {:.0}%", volume_settings.volume * 100.0);
        // 这里可以更新音频系统的音量
        // audio_sink.set_volume(volume_settings.volume);
    }
}

// ==================== 按钮交互系统 ====================

fn handle_button_interactions(
    mut commands: Commands,
    mut popup_state: ResMut<PopupState>,
    font_handle: Res<MainFontHandle>,
    volume_settings: Res<VolumeSettings>,
    btn_queries: (
        Query<&Interaction, (Changed<Interaction>, With<BtnOptions>)>,
        Query<&Interaction, (Changed<Interaction>, With<BtnSelStyle>)>,
        Query<&Interaction, (Changed<Interaction>, With<BtnStartGame>)>,
        Query<&Interaction, (Changed<Interaction>, With<BtnRules>)>,
        Query<&Interaction, (Changed<Interaction>, With<ClosePopupButton>)>,
        Query<&Interaction, (Changed<Interaction>, With<QuitGameButton>)>,
    ),
    popup_query: Query<Entity, With<Popup>>,
    mut button_colors: Query<(&Interaction, &mut BackgroundColor), Or<(
        With<BtnOptions>, 
        With<BtnSelStyle>, 
        With<BtnStartGame>, 
        With<BtnRules>,
        With<QuitGameButton>,
    )>>,
    mut quit_event_writer: EventWriter<bevy::app::AppExit>,
) {
    // 处理按钮颜色变化
    for (interaction, mut color) in button_colors.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.3));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgba(0.3, 0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0));
            }
        }
    }
    
    // 处理选项按钮 - 只在没有弹窗时响应
    if popup_state.active_popup.is_none() {
        for interaction in btn_queries.0.iter() {
            if *interaction == Interaction::Pressed {
                // 清理现有弹窗
                for entity in popup_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                
                // 创建弹窗容器并添加设置内容
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5)),
                        z_index: ZIndex::Global(100),
                        ..default()
                    },
                    Popup,
                    BlockEvents,
                ))
                .with_children(|parent| {
                    create_settings_popup(parent, font_handle.0.clone(), &volume_settings);
                });
                
                popup_state.active_popup = Some(PopupType::Settings);
            }
        }
    }
    
    // 处理风格选择按钮 - 只在没有弹窗时响应
    if popup_state.active_popup.is_none() {
        for interaction in btn_queries.1.iter() {
            if *interaction == Interaction::Pressed {
                for entity in popup_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5)),
                    z_index: ZIndex::Global(100),
                        ..default()
                    },
                    Popup,
                    BlockEvents,
                ))
                .with_children(|parent| {
                    create_style_select_popup(parent, font_handle.0.clone());
                });
                
                popup_state.active_popup = Some(PopupType::StyleSelect);
            }
        }
    }
    
    // 处理游戏开始按钮 - 只在没有弹窗时响应
    if popup_state.active_popup.is_none() {
        for interaction in btn_queries.2.iter() {
            if *interaction == Interaction::Pressed {
                println!("Game Start button pressed!");
                // 这里可以触发游戏状态切换
            }
        }
    }
    
    // 处理规则按钮 - 只在没有弹窗时响应
    if popup_state.active_popup.is_none() {
        for interaction in btn_queries.3.iter() {
            if *interaction == Interaction::Pressed {
                for entity in popup_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5)),
                        z_index: ZIndex::Global(100),
                        ..default()
                    },
                    Popup,
                    BlockEvents,
                ))
                .with_children(|parent| {
                    create_rules_popup(parent, font_handle.0.clone());
                });
                
                popup_state.active_popup = Some(PopupType::Rules);
            }
        }
    }
    
    // 处理关闭按钮
    for interaction in btn_queries.4.iter() {
        if *interaction == Interaction::Pressed {
            for entity in popup_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            popup_state.active_popup = None;
        }
    }
    
    // 处理退出游戏按钮
    for interaction in btn_queries.5.iter() {
        if *interaction == Interaction::Pressed {
            println!("Quit Game button pressed!");
            quit_event_writer.send(bevy::app::AppExit);
        }
    }
}

// ==================== 事件阻止系统 ====================

fn block_events(
    mut events: EventReader<CursorMoved>,
    popup_query: Query<&Node, With<Popup>>,
) {
    // 如果有弹窗，阻止鼠标移动事件传递
    if !popup_query.is_empty() {
        events.clear();
    }
}

// ==================== 清理函数 ====================

fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<(Entity, &Name)>,
) {
    for (entity, name) in menu_query.iter() {
        if name.as_str() == "MainMenu" {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ==================== 插件实现 ====================

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenu), load_assets)
           .add_systems(Update, (
                setup_main_menu,
                handle_button_interactions,
                start_dragging_slider,
                stop_dragging_slider,
                update_volume_slider,
                update_volume_system,
                block_events,
           ))
           .add_systems(OnExit(MainMenu), cleanup_main_menu);
    }
}