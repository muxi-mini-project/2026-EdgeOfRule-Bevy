use bevy::prelude::*;

#[derive(Component)]
pub struct EasyBtn;

#[derive(Component)]
pub struct MediumBtn;

#[derive(Component)]    
pub struct HardBtn;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Easy Button
    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(19.8),
                height: Val::Percent(9.52),
                position_type: PositionType::Absolute,
                left: Val::Percent(12.0),
                bottom: Val::Percent(20.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor::from(Color::rgb(24.0 / 255.0, 43.0 / 255.0, 56.0 / 255.0)),
            ..Default::default()
        },
        EasyBtn,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "我菜",
                TextStyle {
                    font: asset_server.load("font/font/aLiFont.ttf"), 
                    font_size: 50.0, 
                    color: Color::WHITE
                },
            ),
            ..Default::default()
        });
    });

    // Medium Button
    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(19.8),
                height: Val::Percent(9.52),
                position_type: PositionType::Absolute,
                left: Val::Percent(40.1),
                bottom: Val::Percent(20.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor::from(Color::rgb(24.0 / 255.0, 43.0 / 255.0, 56.0 / 255.0)),
            ..Default::default()
        },
        MediumBtn,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "我有点牛逼",
                TextStyle {
                    font: asset_server.load("font/font/aLiFont.ttf"), 
                    font_size: 50.0, 
                    color: Color::WHITE
                },
            ),
            ..Default::default()
        });
    });

    // Hard Button
    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(19.8),
                height: Val::Percent(9.52),
                position_type: PositionType::Absolute,
                right: Val::Percent(12.0),
                bottom: Val::Percent(20.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor::from(Color::rgb(24.0 / 255.0, 43.0 / 255.0, 56.0 / 255.0)),
            ..Default::default()
        },
        HardBtn,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "吔屎啦你",
                TextStyle {
                    font: asset_server.load("font/font/aLiFont.ttf"), 
                    font_size: 50.0, 
                    color: Color::WHITE
                },
            ),
            ..Default::default()
        });
    });
}

pub fn despawn(
    mut commands: Commands,
    easy_btns: Query<Entity, With<EasyBtn>>,
    medium_btns: Query<Entity, With<MediumBtn>>,
    hard_btns: Query<Entity, With<HardBtn>>,
) {
    for btn in &easy_btns {
        commands.entity(btn).despawn_recursive();
    }
    for btn in &medium_btns {
        commands.entity(btn).despawn_recursive();
    }
    for btn in &hard_btns {
        commands.entity(btn).despawn_recursive();
    }
}