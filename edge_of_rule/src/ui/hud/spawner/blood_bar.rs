use bevy::prelude::*;
use bevy::sprite::Anchor; 

use crate::assets::hud::HudImageAssets;
use crate::core::health::{PlayerHealth, PlayerDied, SewageDamageAccum};

#[derive(Component)]
pub struct BloodBar;
#[derive(Component)]
pub struct BloodFill;

pub fn spawn_blood_bar(
    mut commands: Commands, 
    hud_image_assets: Res<HudImageAssets>,
    player_health: Res<PlayerHealth>, 
) {
    commands.insert_resource(PlayerHealth::default());
    let health_percent = player_health.current as f32 / player_health.max as f32;
    
    commands.spawn((
        SpriteBundle {
            texture: hud_image_assets.blood_bar.clone(),
            sprite: Sprite {
                anchor: Anchor::CenterLeft,  
                custom_size: Some(Vec2::new(200.0, 20.0)), 
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-(1550.0/2.0) + 170.0, (950.0/2.0) - 120.0, 99.0),
                scale: Vec3::new(health_percent, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        BloodBar,
    ));
}

pub fn despawn_blood_bar(mut commands: Commands, blood_bars: Query<Entity, With<BloodBar>>) {
    for entity in blood_bars.iter() {
        commands.entity(entity).despawn();
    }
}