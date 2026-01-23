use bevy::prelude::*;

#[derive(Component)]
pub struct GameStartBtn;

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
            GameStartBtn,
        ))
        .with_children(|root| {
            root.spawn(
                (
                    ButtonBundle{
                        style: Style{
                            width: Val::Percent(19.8),
                            height: Val::Percent(9.52),
                            margin: UiRect::bottom(Val::Percent(17.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    GameStartBtn,
                )
            ).with_children(|root| {
                root.spawn((
                TextBundle::from_section(
                    "Levels",
                    TextStyle {
                        font: asset_server.load("font/font/aLiFont.ttf"), 
                        font_size: 55.0, 
                        color: Color::WHITE
                    },
                    
                ),
                // GameStartBtn,
                Interaction::None,
            ));
            })
            ;
            // 
        });
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<GameStartBtn>>) {
    for btn in &btns {
        commands.entity(btn).despawn_recursive();
    }
}
