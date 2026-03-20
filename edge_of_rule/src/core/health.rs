use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerHealth {
    pub current: i32,
    pub max: i32,
    pub dead: bool,
}

impl Default for PlayerHealth {
    fn default() -> Self {
        Self {
            current: 100,
            max: 100,
            dead: false,
        }
    }
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct SewageDamageAccum {
    pub seconds: f32,
}

#[derive(Event, Debug, Default, Clone, Copy)]
pub struct PlayerDied;
