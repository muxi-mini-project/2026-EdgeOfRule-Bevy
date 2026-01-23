use::bevy::prelude::*;

#[derive(Component)]
pub struct ExitGameBtn;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            background_color: BackgroundColor::from(Color::rgb(82.0 / 255.0, 4.0 / 255.0, 4.0 / 255.0)),
            ..Default::default()
        },
        // Interaction::None,
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
    });
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<ExitGameBtn>>) {
    for btn in &btns {
        commands.entity(btn).despawn_recursive();
    }
}
