use bevy::prelude::*;
use crate::assets::ui_image::UiImageAssets;

#[derive(Resource, Default)]
pub struct LevelOneLock {
    pub locked: bool,
}

#[derive(Component)]
pub struct OneLock;

#[derive(Resource, Default)]
pub struct LevelTwoLock {
    pub locked: bool,
}  

#[derive(Component)]
pub struct TwoLock;

#[derive(Resource, Default)]
pub struct LevelThreeLock {
    pub locked: bool,
}

#[derive(Component)]
pub struct ThreeLock;

#[derive(Resource, Default)]
pub struct LevelFourLock {
    pub locked: bool,
}

#[derive(Component)]
pub struct FourLock;

#[derive(Resource, Default)]
pub struct LevelFiveLock {  
    pub locked: bool,
}

#[derive(Component)]
pub struct FiveLock;

#[derive(Resource, Default)]
pub struct LevelSixLock {
    pub locked: bool,
}

#[derive(Component)]
pub struct SixLock;

#[derive(Resource, Default)]
pub struct LevelSevenLock {
    pub locked: bool,
}

#[derive(Component)]
pub struct SevenLock;

pub fn spawner_level_one_lock(
    mut commands: Commands,
    assets: Res<UiImageAssets>,
) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(66.0),
                height: Val::Px(66.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(23.125),
                left: Val::Percent(22.57),
                ..Default::default()
            },
            image: UiImage::from(assets.key_grey.clone()),
            z_index: ZIndex::Global(98),
            ..Default::default()
        },
        OneLock,
    ));
}

pub fn spawner_level_two_lock(
    mut commands: Commands,
    assets: Res<UiImageAssets>,
) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(66.0),
                height: Val::Px(66.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(52.25),
                left: Val::Percent(22.57),
                ..Default::default()
            },
            image: UiImage::from(assets.key_grey.clone()),
            z_index: ZIndex::Global(98),
            ..Default::default()
        },
        TwoLock,
    ));
}

pub fn spawner_level_three_lock(
    mut commands: Commands,
    assets: Res<UiImageAssets>,
) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(66.0),
                height: Val::Px(66.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(81.5625),
                left: Val::Percent(22.57),
                ..Default::default()
            },
            image: UiImage::from(assets.key_grey.clone()),
            z_index: ZIndex::Global(98),
            ..Default::default()
        },
        ThreeLock,
    ));
}

pub fn spawner_level_four_lock(
    mut commands: Commands,
    assets: Res<UiImageAssets>,
) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(66.0),
                height: Val::Px(66.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(23.125),
                right: Val::Percent(22.266),
                ..Default::default()
            },
            image: UiImage::from(assets.key_grey.clone()),
            z_index: ZIndex::Global(98),
            ..Default::default()
        },
        FourLock,
    ));
}

pub fn spawner_level_five_lock(
    mut commands: Commands,
    assets: Res<UiImageAssets>,
) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(66.0),
                height: Val::Px(66.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(52.25),
                right: Val::Percent(22.266),
                ..Default::default()
            },
            image: UiImage::from(assets.key_grey.clone()),
            z_index: ZIndex::Global(98),
            ..Default::default()
        },
        FiveLock,
    ));
}

pub fn spawner_level_six_lock(
    mut commands: Commands,
    assets: Res<UiImageAssets>,
) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(66.0),
                height: Val::Px(66.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(81.5625),
                right: Val::Percent(22.266),
                ..Default::default()
            },
            image: UiImage::from(assets.key_grey.clone()),
            z_index: ZIndex::Global(98),
            ..Default::default()
        },
        SixLock,
    ));
}

pub fn spawner_level_seven_lock(
    mut commands: Commands,
    assets: Res<UiImageAssets>,
) {
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(66.0),
                height: Val::Px(66.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(64.375),
                left: Val::Percent(48.055),
                ..Default::default()
            },
            image: UiImage::from(assets.key_grey.clone()),
            z_index: ZIndex::Global(98),
            ..Default::default()
        },
        SevenLock,
    ));
}

pub fn despawn(mut commands: Commands, btns: Query<Entity, Or<(With<OneLock>, With<TwoLock>, With<ThreeLock>, With<FourLock>, With<FiveLock>, With<SixLock>, With<SevenLock>)>>) {
    for btn in &btns {
        commands.entity(btn).despawn_recursive();
    }
}