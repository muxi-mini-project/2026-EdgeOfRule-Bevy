use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum CollisionGroup {
    Player = 1,
    Ground = 2,
}

impl From<CollisionGroup> for Group {
    fn from(value: CollisionGroup) -> Self {
        Group::from_bits(1 << (value as u32)).unwrap()
    }
}
