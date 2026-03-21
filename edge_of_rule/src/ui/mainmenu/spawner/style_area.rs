use bevy::prelude::*;

#[derive(Component)]
pub struct StyleArea;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ))
    .with_children(|root| {
        root.spawn((
            ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                image: UiImage::new(asset_server.load("images/mainmenu/style_area.png")),
                ..Default::default()
            },
            StyleArea,
        ));
    });
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<StyleArea>>) {
    for btn in &btns {
        commands.entity(btn).despawn_recursive();
    }
}