use bevy::prelude::*;

use crate::assets::hud::HudImageAssets;
use crate::core::state::GameState;

#[derive(Component)]
pub struct BloodBar;

pub fn manage_blood_bar(
    mut commands: Commands,
    state: Res<State<GameState>>,
    mut blood_bar_entity: Local<Option<Entity>>,
    hud_image_assets: Res<HudImageAssets>,
) {
    use GameState::*;
    let in_gameplay = matches!(
        state.get(),
        Day1Scene1
            | Day1Scene2
            | Day1Scene3
            | Day1Scene4
            | Day2Scene1
            | Day2Scene2
            | Day2Scene3
            | Day2Scene4
            | Day2Scene5
    );

    if in_gameplay && blood_bar_entity.is_none() {
        let entity = commands
            .spawn((
                ImageBundle {
                    image: UiImage::new(hud_image_assets.blood_bar.clone()),
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(20.0),
                        top: Val::Px(20.0),
                        ..Default::default()
                    },
                    z_index: ZIndex::Global(500),
                    ..Default::default()
                },
                BloodBar,
            ))
            .id();
        *blood_bar_entity = Some(entity);
    } else if !in_gameplay && blood_bar_entity.is_some() {
        commands
            .entity(blood_bar_entity.unwrap())
            .despawn_recursive();
        *blood_bar_entity = None;
    }
}
