use bevy::prelude::*;

#[derive(Component)]
pub struct StyleSelectBtn;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            StyleSelectBtn,
        ))
        .with_children(|root| {
            root.spawn(
                (
                    ButtonBundle{
                        style: Style{
                            width: Val::Percent(19.8),
                            height: Val::Percent(9.52),
                            margin: UiRect::bottom(Val::Percent(27.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    StyleSelectBtn,
                )
            ).with_children(|root| {
                root.spawn((
                TextBundle::from_section(
                    "Select Style",
                    TextStyle {
                        font: asset_server.load("font/font/aLiFont.ttf"), 
                        font_size: 55.0, 
                        color: Color::WHITE
                    },
                    
                ),
                Interaction::None,
            ));
            })
            ;
            // 
        });
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<StyleSelectBtn>>) {
    for btn in &btns {
        commands.entity(btn).despawn_recursive();
    }
}
