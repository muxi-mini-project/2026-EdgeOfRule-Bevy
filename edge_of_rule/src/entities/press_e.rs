use bevy::prelude::*;

use crate::constants::SCALE;

#[derive(Component)]
pub struct PressE;

pub fn spawn_press_e(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &Res<AssetServer>,
    content: &str,
    component: impl Component,
) {
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("images/HUD/press_e.png"),
                transform: transform.with_scale(Vec3::splat(SCALE)),
                ..default()
            },
            PressE,
            component,
        ))
        .with_children(|root| {
            root.spawn(Text2dBundle {
                text: Text::from_section(
                    content,
                    TextStyle {
                        font: asset_server.load("font/font/aLiFont.ttf"),
                        font_size: 6.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(0.0, -10.0, 0.0),
                ..Default::default()
            });
        });
}
