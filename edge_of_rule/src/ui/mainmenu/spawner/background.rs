use bevy::prelude::*;

use crate::assets::ui_image::UiImageAssets;

#[derive(Component)]
pub struct Background;

pub fn spawn(mut commands: Commands, assets: Res<UiImageAssets>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            image: UiImage::new(assets.background.clone()),
            z_index: ZIndex::Global(-1),
            ..Default::default()
        },
        Background,
    ));
}

pub fn despawn(mut commands: Commands, backgrounds: Query<Entity, With<Background>>) {
    for background in &backgrounds {
        commands.entity(background).despawn();
    }
}
