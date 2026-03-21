use bevy::prelude::*;

#[derive(Component)]
pub struct EasyBtn;

#[derive(Component)]
pub struct MediumBtn;

#[derive(Component)]    
pub struct HardBtn;

#[derive(Component, Default)]
pub struct StyleBtnState {
    pub pressed: bool,
}

#[derive(Resource, Default)]
pub struct ButtonSelectionState {
    pub is_selected: bool,
    pub selected_btn: Option<String>,
}

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ButtonSelectionState::default());

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
        StyleBtnState { pressed: false },
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
        StyleBtnState { pressed: false },
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
        StyleBtnState { pressed: false },
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