use bevy::prelude::*;

use crate::animation::lift_door::LiftDoorAnim;

pub fn start_close_lift_on_enter(mut lift_door: ResMut<LiftDoorAnim>) {
    lift_door.start_close();
}
