use bevy::prelude::*;

#[derive(Component)]
pub struct LevelDay1;

#[derive(Component)]
pub struct LevelDay2;

#[derive(Component)]
pub struct LevelDay3;

#[derive(Component)]
pub struct LevelDay4;

#[derive(Component)]
pub struct LevelDay5;

#[derive(Component)]
pub struct LevelDay6;

#[derive(Component)]
pub struct LevelDay7;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(16.09),
                height: Val::Percent(24.57),
                position_type: PositionType::Absolute,
                top: Val::Percent(8.54),
                left: Val::Percent(16.83),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/day_1.png")),
            ..Default::default()
        },
        LevelDay1,
        Interaction::None,
    ));
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(16.09),
                height: Val::Percent(24.57),
                position_type: PositionType::Absolute,
                top: Val::Percent(37.715),
                left: Val::Percent(16.83),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/day_2.png")),
            ..Default::default()
        },
        LevelDay2,
        Interaction::None,
    ));
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(16.09),
                height: Val::Percent(24.57),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(8.54),
                left: Val::Percent(16.83),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/day_3.png")),
            ..Default::default()
        },
        LevelDay3,
        Interaction::None,
    ));
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(16.09),
                height: Val::Percent(24.57),
                position_type: PositionType::Absolute,
                top: Val::Percent(8.54),
                right: Val::Percent(16.83),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/day_4.png")),
            ..Default::default()
        },
        LevelDay4,
        Interaction::None,
    ));
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(16.09),
                height: Val::Percent(24.57),
                position_type: PositionType::Absolute,
                top: Val::Percent(37.715),
                right: Val::Percent(16.83),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/day_5.png")),
            ..Default::default()
        },
        LevelDay5,
        Interaction::None,
    ));
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(16.09),
                height: Val::Percent(24.57),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(8.54),
                right: Val::Percent(16.83),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/day_6.png")),
            ..Default::default()
        },
        LevelDay6,
        Interaction::None,
    ));
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(16.09),
                height: Val::Percent(55.05),
                position_type: PositionType::Absolute,
                top: Val::Percent(22.6),
                left: Val::Percent(41.95),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/day_7.png")),
            ..Default::default()
        },
        LevelDay7,
        Interaction::None,
    ));
}

pub fn despawn(mut commands: Commands, levels: Query<Entity, Or<(
    With<LevelDay1>,
    With<LevelDay2>,
    With<LevelDay3>,
    With<LevelDay4>,
    With<LevelDay5>,
    With<LevelDay6>,
    With<LevelDay7>,
)>>) {
    for level in &levels {
        commands.entity(level).despawn();
    }
}