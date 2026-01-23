use bevy::prelude::*;

#[derive(Component)]
pub struct StyleArea;

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(70.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(15.0),
                left: Val::Percent(10.0),
                ..Default::default()
            },
            z_index: ZIndex::Global(0),
            background_color: BackgroundColor::from(Color::rgb(0.2, 0.2, 0.3)),
            ..Default::default()
        },
        StyleArea,
    ));
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, With<StyleArea>>) {
    for btn in &btns {
        commands.entity(btn).despawn_recursive();
    }
}