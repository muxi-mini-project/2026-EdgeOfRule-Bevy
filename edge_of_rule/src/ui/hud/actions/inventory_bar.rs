use bevy::prelude::*;

use crate::core::inventory::Inventory;

pub fn select_slot(input: Res<ButtonInput<KeyCode>>, mut inventory: ResMut<Inventory>) {
    let keys = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
    ];
    for (i, key) in keys.iter().enumerate() {
        if input.just_pressed(*key) {
            inventory.selected = i;
        }
    }
}
