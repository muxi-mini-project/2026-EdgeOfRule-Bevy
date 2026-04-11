use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Resource, Default)]
pub struct WarningPopupState {
    pub structed: bool,
}

#[derive(Component)]
pub struct WarningPopup;

pub fn spawn(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(90),
            background_color: BackgroundColor::from(Color::rgba(0.0, 0.0, 0.0, 0.7)),
            ..Default::default()
        },
        WarningPopup,
    ))
    .with_children(|parent| {
        parent.spawn(
            TextBundle {
                text: Text::from_section(
                    "请先完成前面的关卡！",
                    TextStyle {
                        font: assets.load("font/font/aLiFont.ttf"),
                        font_size: 70.0,
                        color: Color::rgb(208.0 / 255.0, 2.0 / 255.0, 32.0 / 255.0),
                    },
                ),
                ..Default::default()
            },
        );
    });
}

pub fn update_popup_visibility(
    mut query: Query<&mut Visibility, With<WarningPopup>>,
    state: Res<WarningPopupState>,
) {
    if let Ok(mut visibility) = query.get_single_mut() {
        if state.structed {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn timer(
    mut state: ResMut<WarningPopupState>,
    timer: Res<Time>,
    mut time: Local<Timer>,
) {
    if state.structed {
        if time.duration().is_zero() {
            time.set_duration(std::time::Duration::from_secs_f32(1.2));
            time.reset();
        }

        time.tick(timer.delta());

        if time.just_finished() {
            state.structed = false;
            time.reset();
        } 
    } else {
            time.reset();
    }
}

pub fn despawn(mut commands: Commands, popups: Query<Entity, With<WarningPopup>>) {
    for popup in &popups {
        commands.entity(popup).despawn_recursive();
    }
}

