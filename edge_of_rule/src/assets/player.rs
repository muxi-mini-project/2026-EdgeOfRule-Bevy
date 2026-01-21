use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct PlayerAssets {
    pub front_texture: Handle<Image>,
    pub side_textures: Vec<Handle<Image>>,
    pub front_squat_texture: Handle<Image>,
    pub side_squat_textures: Vec<Handle<Image>>,
    pub slide_textures: [Handle<Image>; 2],
}

pub fn load_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerAssets {
        front_texture: asset_server.load("images/player.png"),
        side_textures: vec![
            asset_server.load("images/player_side_0.png"),
            asset_server.load("images/player_side_1.png"),
            asset_server.load("images/player_side_2.png"),
            asset_server.load("images/player_side_3.png"),
        ],
        front_squat_texture: asset_server.load("images/player_squat.png"),
        side_squat_textures: vec![
            asset_server.load("images/player_squat_side_0.png"),
            asset_server.load("images/player_squat_side_1.png"),
        ],
        slide_textures: [
            asset_server.load("images/slide_0.png"),
            asset_server.load("images/slide_1.png"),
        ],
    });
}
