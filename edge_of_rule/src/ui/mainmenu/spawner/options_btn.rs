use bevy::prelude::*;

#[derive(Component)]
pub struct OptionBtn(pub bool);

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(4.73),
                height: Val::Percent(7.86),
                position_type: PositionType::Absolute,
                top: Val::Percent(2.0),
                left: Val::Percent(1.89),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/options_btn.png")),
            z_index: ZIndex::Global(0),
            ..Default::default()
        },
        OptionBtn(false),
        Interaction::None,
    ));
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<OptionBtn>>) {
    for btn in &btns {
        commands.entity(btn).despawn();
    }
}
