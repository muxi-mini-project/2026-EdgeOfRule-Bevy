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
            root.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Percent(25.0),
                        height: Val::Percent(9.0),
                        margin: UiRect::bottom(Val::Percent(20.0)),
                        ..Default::default()
                    },
                    image: UiImage::new(asset_server.load("images/mainmenu/game_start_btn.png")),
                    ..Default::default()
                },
                GameStartBtn,
                Interaction::None,
            ));
        });
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<GameStartBtn>>) {
    for btn in &btns {
        commands.entity(btn).despawn();
    }
}
