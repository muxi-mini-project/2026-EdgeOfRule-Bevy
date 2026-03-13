// use bevy::prelude::*;

// #[derive(Component)]
// pub struct InGameOptionsArea;

// // 版本 1：作为 Bevy 系统的版本
// pub fn spawn_system(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     spawn_internal(&mut commands, &asset_server);
// }


// pub fn spawn_internal(
//     commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
// ) {
//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     width: Val::Percent(60.0),
//                     height: Val::Percent(70.0),
//                     justify_content: JustifyContent::End,
//                     align_items: AlignItems::Center,
//                     position_type: PositionType::Absolute,
//                     top: Val::Percent(15.0),
//                     left: Val::Percent(20.0),
//                     flex_direction: FlexDirection::Column,
//                     ..Default::default()
//                 },
//                 z_index: ZIndex::Global(1),
//                 background_color: BackgroundColor::from(Color::rgb(0.2, 0.2, 0.3)),
//                 ..Default::default()
//             },
//             InGameOptionsArea,
//         ));
// }

// // 保持原来的 spawn 函数作为兼容（如果需要）
// pub fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>) {
//     spawn_internal(commands, asset_server);
// }

// pub fn despawn(
//     mut commands: Commands,
//     btns: Query<Entity, With<InGameOptionsArea>>,
// ) {
//     for btn in btns.iter() {
//         commands.entity(btn).despawn_recursive();
//     }
// }