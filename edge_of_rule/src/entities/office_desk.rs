use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct OfficeDesk;

pub fn spawn_office_desk(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("images/animations/office_desk.png"),
                transform: transform.with_scale(Vec3::splat(SCALE)),
                ..default()
            },
            OfficeDesk,
        ))
        .id()
}
