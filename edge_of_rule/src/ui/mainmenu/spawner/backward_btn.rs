use bevy::prelude::*;

#[derive(Component)]
pub struct BackwardBtn;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(4.73),
                height: Val::Percent(7.86),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(2.0),
                left: Val::Percent(1.89),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/backward_btn.png")),
            z_index: ZIndex::Global(0),
            ..Default::default()
        },
        BackwardBtn,
        Interaction::None,
    ));
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<BackwardBtn>>) {
    for btn in &btns {
        commands.entity(btn).despawn();
    }
}
