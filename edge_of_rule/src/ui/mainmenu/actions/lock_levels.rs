use bevy::{ecs::query, prelude::*};
use crate::{assets::ui_image::UiImageAssets, core::state::GameState, ui::mainmenu::spawner::lock_levels::{
    LevelOneLock, LevelTwoLock, LevelThreeLock, LevelFourLock, LevelFiveLock, LevelSixLock, LevelSevenLock, 
    OneLock,      TwoLock,      ThreeLock,      FourLock,      FiveLock,      SixLock,      SevenLock,
}};

pub fn unlock_levels_one(
    mut level_one_lock: ResMut<LevelOneLock>,
    mut query: Query<&mut BackgroundColor, With<OneLock>>,
) {
    level_one_lock.locked = false;

    if level_one_lock.locked {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::WHITE);
        }
    } else  {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::RED);
        }
    }
}

pub fn unlock_levels_two(
    mut level_two_lock: ResMut<LevelTwoLock>,
    mut query: Query<&mut BackgroundColor, With<TwoLock>>,
) {
    if level_two_lock.locked {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::WHITE);
        }
    } else  {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::RED);
        }
    }
}

pub fn unlock_levels_three(
    mut level_three_lock: ResMut<LevelThreeLock>,
    mut query: Query<&mut BackgroundColor, With<ThreeLock>>,
) {
    if level_three_lock.locked {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::WHITE);
        }
    } else  {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::RED);
        }
    }
}

pub fn unlock_levels_four(
    mut level_four_lock: ResMut<LevelFourLock>,
    mut query: Query<&mut BackgroundColor, With<FourLock>>,
) {

    if level_four_lock.locked {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::WHITE);
        }
    } else  {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::RED);
        }
    }
}

pub fn unlock_levels_five(
    mut level_five_lock: ResMut<LevelFiveLock>,
    mut query: Query<&mut BackgroundColor, With<FiveLock>>,
) {

    if level_five_lock.locked {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::WHITE);
        }
    } else  {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::RED);
        }
    }
}

pub fn unlock_levels_six(
    mut level_six_lock: ResMut<LevelSixLock>,
    mut query: Query<&mut BackgroundColor, With<SixLock>>,
) {

    if level_six_lock.locked {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::WHITE);
        }
    } else  {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::RED);
        }
    }
}

pub fn unlock_levels_seven(
    mut level_seven_lock: ResMut<LevelSevenLock>,
    mut query: Query<&mut BackgroundColor, With<SevenLock>>,
) {

    if level_seven_lock.locked {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::WHITE);
        }
    } else  {
        for mut color in &mut query {
            *color = BackgroundColor::from(Color::RED);
        }
    }
}

