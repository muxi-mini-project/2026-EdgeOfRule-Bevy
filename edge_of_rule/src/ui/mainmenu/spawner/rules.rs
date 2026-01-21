use bevy::prelude::*;

use crate::assets::ui_image::UiImageAssets;

#[derive(Component)]
pub struct Rules;

pub fn spawn(mut commands: Commands, asset: Res<UiImageAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            Rules,
        ))
        .with_children(|root| {
            root.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Percent(75.0),
                        height: Val::Percent(75.0),
                        ..Default::default()
                    },
                    image: UiImage::new(asset.rules.clone()),
                    ..Default::default()
                },
                Rules,
            ));
        });
}

pub fn despawn(mut commands: Commands, entities: Query<Entity, With<Rules>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
