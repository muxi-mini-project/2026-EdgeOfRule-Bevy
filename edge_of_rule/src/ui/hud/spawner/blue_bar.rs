use bevy::prelude::*;

// pub fn spawn_blue_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let blue_handle = asset_server.load("images/HUD/blue.png");
//     let blue_bar_handle = asset_server.load("images/HUD/blue_bar.png");
//     commands.spawn_bundle(NodeBundle {
//         transform: Transform {
//             translation: Vec3::new(0.0, -200.0, 0.0),
//             scale: Vec3::new(1.0, 1.0, 1.0),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// }