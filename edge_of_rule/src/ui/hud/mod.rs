pub mod actions;
pub mod spawner;

use crate::ui::mainmenu;

use bevy::prelude::*;

use crate::core::state::GameState;

pub struct HudPlugin;

// impl Plugin for HudPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .add_systems(
//                 OnEnter(GameState::Day1Scene1),
//                 (

//                 ),
//             )
//             .add_systems(
//                 OnExit(GameState::Day1Scene1),
//                 (

//                 ),
//             )
//             .add_systems(
//                 OnEnter(GameState::Day1Scene2),
//                 (

//                 ),
//             )
//             .add_systems(
//                 OnExit(GameState::Day1Scene2),
//                 (

//                 ),
//             )
//             .add_systems(
//                 OnEnter(GameState::InGameOption),
//                 (
//                 )
//             )
//             .add_systems(
//                 OnExit(GameState::InGameOption),
//                 (
//                 )
//             )
//             .add_systems(
//                 Update,
//                 (
//                 )
//             );
//     }
// }   