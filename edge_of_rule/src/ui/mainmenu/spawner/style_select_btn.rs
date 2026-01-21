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
            root.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Percent(25.0),
                        height: Val::Percent(9.0),
                        margin: UiRect::bottom(Val::Percent(30.0)),
                        ..Default::default()
                    },
                    image: UiImage::new(asset_server.load("images/mainmenu/style_select_btn.png")),
                    ..Default::default()
                },
                StyleSelectBtn,
                Interaction::None,
            ));
        });
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<StyleSelectBtn>>) {
    for btn in &btns {
        commands.entity(btn).despawn();
    }
}
