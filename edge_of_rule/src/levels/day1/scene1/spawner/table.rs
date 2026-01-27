use bevy::prelude::*;

use crate::entities::table::{spawn_table, Table};

pub fn spawn(mut commands: Commands, asset: Res<AssetServer>) {
    spawn_table(&mut commands, Transform::from_xyz(8.0, -68.0, -5.0), asset);
}

pub fn despawn(mut commands: Commands, tables: Query<Entity, With<Table>>) {
    for table in &tables {
        commands.entity(table).despawn();
    }
}
