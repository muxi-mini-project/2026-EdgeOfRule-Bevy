use bevy::prelude::*;

use crate::assets::ui_image::UiImageAssets;

#[derive(Component)]
pub struct StyleArea;

pub fn spawn(mut commands: Commands, asset_server: Res<UiImageAssets>) {
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
                image: UiImage::new(asset_server.style_area.clone()),
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