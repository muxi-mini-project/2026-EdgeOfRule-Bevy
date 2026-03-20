use bevy::prelude::*;

use crate::core::state::GameState;

#[derive(Eq, PartialEq)]
enum FadeMode {
    In,
    Out,
}

#[derive(Component)]
pub struct FadeMask {
    timer: Timer,
    mode: FadeMode,
    next_state: GameState,
}

pub fn spawn_mask(commands: &mut Commands, next_state: GameState) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            z_index: ZIndex::Global(i32::MAX),
            ..Default::default()
        },
        FadeMask {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            mode: FadeMode::Out,
            next_state,
        },
    ));
}

pub fn fade_animation(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FadeMask, &mut BackgroundColor)>,
) {
    for (entity, mut mask, mut background_color) in query.iter_mut() {
        mask.timer.tick(time.delta());

        if mask.timer.just_finished() {
            if mask.mode == FadeMode::Out {
                mask.mode = FadeMode::In;
                mask.timer = Timer::from_seconds(1.0, TimerMode::Once);
                game_state.set(mask.next_state);
            } else {
                commands.entity(entity).despawn();
                return;
            }
        }

        let progress = mask.timer.fraction();
        match mask.mode {
            FadeMode::Out => background_color.0.set_a(progress),
            FadeMode::In => background_color.0.set_a(1.0 - progress),
        };
    }
}
