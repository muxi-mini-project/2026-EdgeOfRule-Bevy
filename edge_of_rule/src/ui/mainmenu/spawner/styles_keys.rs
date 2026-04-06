use bevy::prelude::*;

#[derive(Component)]
pub struct EasyBtn;

#[derive(Component)]
pub struct MediumBtn;

#[derive(Component)]    
pub struct HardBtn;

#[derive(Resource, Default)]
pub struct HardBtnState {
    pub pressed: bool,
}

#[derive(Resource, Default)]
pub struct MidBtnState {
    pub pressed: bool,
}

#[derive(Resource, Default)]
pub struct EasyBtnState {
    pub pressed: bool,
}

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(4.73),
                height: Val::Percent(7.86),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(14.180),
                left: Val::Percent(16.712),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/key_grey.png")),
            z_index: ZIndex::Global(3),
            ..Default::default()
        },
        EasyBtn,
        Interaction::None,
    ));

    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(4.73),
                height: Val::Percent(7.86),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(14.180),
                left: Val::Percent(48.038),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/key_grey.png")),
            z_index: ZIndex::Global(3),
            ..Default::default()
        },
        MediumBtn,
        Interaction::None,
    ));

    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(4.73),
                height: Val::Percent(7.86),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(14.180),
                right: Val::Percent(16.712),
                ..Default::default()
            },
            image: UiImage::new(asset_server.load("images/mainmenu/key_grey.png")),
            z_index: ZIndex::Global(3),
            ..Default::default()
        },
        HardBtn,
        Interaction::None,
    ));
}

pub fn despawn(
    mut commands: Commands,
    easy_btns: Query<Entity, With<EasyBtn>>,
    medium_btns: Query<Entity, With<MediumBtn>>,
    hard_btns: Query<Entity, With<HardBtn>>,
) {
    for btn in &easy_btns {
        commands.entity(btn).despawn_recursive();
    }
    for btn in &medium_btns {
        commands.entity(btn).despawn_recursive();
    }
    for btn in &hard_btns {
        commands.entity(btn).despawn_recursive();
    }
}