use bevy::prelude::*;

#[derive(Component)]
pub struct InGameOptionArea;

#[derive(Component)]
pub struct InGameOptionTitle;

#[derive(Component)]
pub struct UnderTip;

#[derive(Component)]
pub struct InGameExitBtn;

#[derive(Component)]
pub struct BackToMainMenuBtn;

// 在启动时生成 UI（需要在插件中添加）
pub fn spawn_ingame_option_area(
    mut commands: Commands,
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
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(99),
            background_color: BackgroundColor::from(Color::rgb(0.2, 0.2, 0.3)),
            ..Default::default()
        },
        InGameOptionArea,
        
    ));
}

pub fn spawn_in_game_option_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "暂停游戏",
                TextStyle {
                    font: asset_server.load("font/font/aLiFont.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                width: Val::Percent(16.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(20.0),
                left: Val::Percent(46.0),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(999),
            ..Default::default()
        },
        InGameOptionTitle,
        
    ));
}

pub fn spawn_exit_game_btn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
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
            z_index: ZIndex::Global(999),
            visibility: Visibility::Hidden,
            background_color: BackgroundColor::from(Color::rgb(82.0 / 255.0, 4.0 / 255.0, 4.0 / 255.0)),
            ..Default::default()
        },
        InGameExitBtn,
        
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
    });
}

pub fn spawn_under_tip(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "按 Esc 键继续游戏",
                TextStyle {
                    font: asset_server.load("font/font/aLiFont.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                width: Val::Percent(15.0),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(33.0),
                left: Val::Percent(45.0),
                ..Default::default()
            },
            z_index: ZIndex::Global(999),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        UnderTip,
    ));
}

pub fn spawn_back_to_mainmenu_btn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(19.8),
                height: Val::Percent(8.52),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                bottom: Val::Percent(30.0),
                right: Val::Percent(40.1),
                ..Default::default()
            },
            z_index: ZIndex::Global(999),
            visibility: Visibility::Hidden,
            background_color: BackgroundColor::from(Color::rgb(82.0 / 255.0, 4.0 / 255.0, 4.0 / 255.0)),
            ..Default::default()
        },
        BackToMainMenuBtn,
        
    ))
    .with_children(|parent| {
        parent.spawn(
            TextBundle {
                text: Text::from_section(
                    "返回主菜单",
                    TextStyle {
                        font: asset_server.load("font/font/aLiFont.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            },
        );
    });
}
