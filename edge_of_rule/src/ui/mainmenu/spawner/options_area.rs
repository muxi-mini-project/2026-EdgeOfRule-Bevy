use bevy::prelude::*;

#[derive(Component)]
pub struct OptionsArea;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(60.0),
                    height: Val::Percent(70.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Percent(15.0),
                    left: Val::Percent(20.0),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: BackgroundColor::from(Color::rgb(0.2, 0.2, 0.3)),
                ..Default::default()
            },
            OptionsArea,
        ))
        ;
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<OptionsArea>>) {
    for btn in &btns {
        commands.entity(btn).despawn_recursive();
    }
}
