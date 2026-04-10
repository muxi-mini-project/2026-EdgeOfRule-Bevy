use bevy::prelude::*;

use crate::entities::office_desk::spawn_office_desk;

#[derive(Component)]
pub struct Day2Scene5OfficeDesk;

const POS: Vec3 = Vec3::new(44.0, -44.0, -4.0);

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    let e = spawn_office_desk(&mut commands, Transform::from_translation(POS), &asset);
    commands.entity(e).insert(Day2Scene5OfficeDesk);
}

pub fn despawn(mut commands: Commands, desks: Query<Entity, With<Day2Scene5OfficeDesk>>) {
    for e in &desks {
        commands.entity(e).despawn_recursive();
    }
}
