use bevy::prelude::*;

use crate::ui::mainmenu::spawner::options_btn::OptionBtn;

pub fn spawn(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(8.53),
                height: Val::Percent(4.73),
                position_type: PositionType::Absolute,
                top: Val::Percent(11.0),
                left: Val::Percent(6.62),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/HUD/blood_bar.png")),
            z_index: ZIndex::Global(0),
            ..Default::default()
        },
        Interaction::None,
    ));
}