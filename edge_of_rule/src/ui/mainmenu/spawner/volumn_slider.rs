use bevy::prelude::*;

#[derive(Component)]
pub struct VolumnSlider;  // 整个滑块的标记

#[derive(Component)]
pub struct SliderTrack;    // 轨道标记（蓝色条）

#[derive(Component)]
pub struct SliderHandle;   // 滑块标记（可拖动的按钮）

#[derive(Component)]
pub struct SliderValueText; // 显示百分比的文本

#[derive(Component)]
pub struct SliderValue(pub f32); // 存储当前值 (0.0 - 1.0)


pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(51.56),
                height: Val::Percent(3.13),
                position_type: PositionType::Absolute,
                top: Val::Percent(34.68),
                left: Val::Percent(24.22),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            z_index: ZIndex::Global(1),
            background_color: BackgroundColor::from(Color::rgb(0.3, 0.3, 0.4)),
            ..Default::default()
        },
        VolumnSlider,
    ))
    .with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(70.0),
                    height: Val::Percent(50.0),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(25.0),
                    left: Val::Percent(7.5),
                    ..Default::default()
                },
                background_color: BackgroundColor::from(Color::rgb(0.0, 170.0 / 255.0 , 1.0)),
                ..Default::default()
            },
            SliderTrack,
        ));
    })
    .with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(1.5),
                    height: Val::Percent(80.0),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(10.0),
                    left: Val::Percent(77.5),
                    ..Default::default()
                },
                background_color: BackgroundColor::from(Color::rgb(0.5, 0.5, 0.6)),
                ..Default::default()
            },
            SliderHandle,
            SliderValue(1.0),
        ));

        parent.spawn((
            TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(20.0),
                    left: Val::Percent(2.5),
                    ..Default::default()
                }, 
                text: Text::from_section(
                    "音量",
                    TextStyle {
                        font_size: 20.0,
                        font: asset_server.load("font/font/aLiFont.ttf"),
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            },
        ));

        parent.spawn((
            TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(20.0),
                    left: Val::Percent(90.0),
                    ..Default::default()
                },
                text: Text::from_section(
                    "100%",
                    TextStyle {
                        font_size: 20.0,
                        font: asset_server.load("font/font/aLiFont.ttf"),
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            },
            SliderValueText,
        ));
    });
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<VolumnSlider>>) {
    for btn in &btns {
        commands.entity(btn).despawn_recursive();
    }
}