use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LiftDoorDirection {
    Opening,
    Closing,
}

#[derive(Resource)]
pub struct LiftDoorAnim {
    pub active: bool,
    pub direction: LiftDoorDirection,
    // 0..=2 maps to fixed_1..=fixed_3 depending on direction.
    pub frame: i8,
    pub timer: Timer,
}

impl Default for LiftDoorAnim {
    fn default() -> Self {
        Self {
            active: false,
            direction: LiftDoorDirection::Opening,
            frame: 0,
            timer: Timer::from_seconds(0.12, TimerMode::Repeating),
        }
    }
}

impl LiftDoorAnim {
    pub fn start_open(&mut self) {
        self.active = true;
        self.direction = LiftDoorDirection::Opening;
        self.frame = 0;
        self.timer = Timer::from_seconds(0.12, TimerMode::Repeating);
    }

    pub fn start_close(&mut self) {
        self.active = true;
        self.direction = LiftDoorDirection::Closing;
        self.frame = 2;
        self.timer = Timer::from_seconds(0.12, TimerMode::Repeating);
    }
}

pub fn tick_lift_door_anim(time: Res<Time>, mut anim: ResMut<LiftDoorAnim>) {
    if !anim.active {
        return;
    }

    anim.timer.tick(time.delta());
    if !anim.timer.just_finished() {
        return;
    }

    match anim.direction {
        LiftDoorDirection::Opening => {
            anim.frame = (anim.frame + 1).min(2);
        }
        LiftDoorDirection::Closing => {
            if anim.frame > 0 {
                anim.frame -= 1;
            } else {
                // When fully closed, fall back to the default fixed_0 texture.
                anim.active = false;
            }
        }
    }
}
