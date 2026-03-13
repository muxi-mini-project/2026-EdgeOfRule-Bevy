use bevy::prelude::*;

use crate::ui::mainmenu::spawner::exit_game_btn::ExitGameBtn;

#[derive(Component)]
pub struct InGameOptionsArea;

#[derive(Resource, Default)]
pub struct OptionSpawnState {
    pub is_visible: bool,
}

// 在启动时生成 UI（需要在插件中添加）
pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(60.0),
                height: Val::Percent(70.0),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                top: Val::Percent(15.0),
                left: Val::Percent(20.0),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            z_index: ZIndex::Global(1),
            background_color: BackgroundColor::from(Color::rgb(0.2, 0.2, 0.3)),
            visibility: Visibility::Hidden, 
            ..Default::default()
        },
        InGameOptionsArea,
    ))
    .with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                width: Val::Percent(19.8),
                height: Val::Percent(8.52),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                bottom: Val::Percent(20.0),
                right: Val::Percent(40.1),
                ..Default::default()
            },
            background_color: BackgroundColor::from(Color::rgb(82.0 / 255.0, 4.0 / 255.0, 4.0 / 255.0)),
            ..Default::default()
            },
            ExitGameBtn,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text::from_section(
                        "退出游戏",
                        TextStyle {
                            font: asset_server.load("font/font/aLiFont.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                },
            );
    });});
}

pub fn on_key_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<OptionSpawnState>,
    mut q_menu: Query<&mut Visibility, With<InGameOptionsArea>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        menu_state.is_visible = !menu_state.is_visible;
        
        if let Ok(mut visibility) = q_menu.get_single_mut() {
            *visibility = if menu_state.is_visible {
                info!("显示菜单");
                Visibility::Visible
            } else {
                info!("隐藏菜单");
                Visibility::Hidden
            };
        } else {
            warn!("未找到 InGameOptionsArea 实体");
        }
    }
}