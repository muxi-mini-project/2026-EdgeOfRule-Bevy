use bevy::prelude::*;

use crate::assets::ui_image::UiImageAssets;

#[derive(Component)]
pub struct ArrowRules;

pub fn spawn(mut commands: Commands, assets: Res<UiImageAssets>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(26.0),
                height: Val::Px(43.0),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(15.2),
                left: Val::Percent(35.618),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            image: UiImage::new(assets.choise_triangle.clone()),
            z_index: ZIndex::Global(10),
            ..Default::default()
        },
        ArrowRules,
        Interaction::None,
    ));
}