use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct UiImageAssets {
    pub background: Handle<Image>,
    pub titled_background: Handle<Image>,
    pub rules: Handle<Image>,
}

pub fn load_ui_image_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiImageAssets {
        background: asset_server.load("images/mainmenu/background.png"),
        titled_background: asset_server.load("images/mainmenu/titled_background.png"),
        rules: asset_server.load("images/mainmenu/rules.png"),
    });
}
