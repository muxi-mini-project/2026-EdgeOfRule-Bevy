use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct DoorAssets {
    pub scene1: Handle<Image>,
    pub scene1_opened: Handle<Image>,
    pub scene2: Handle<Image>,
    pub scene3: Handle<Image>,
    pub scene3_opened: Handle<Image>,
    pub trapdoor: Handle<Image>,
    pub trapdoor_opened: Handle<Image>,
}

pub fn load_door_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DoorAssets {
        scene1: asset_server.load("images/animations/door.png"),
        scene1_opened: asset_server.load("images/levels/day1/scene1_opened_door.png"),
        scene2: asset_server.load("images/levels/day1/scene2_opened_door.png"),
        scene3: asset_server.load("images/animations/control_room_door.png"),
        scene3_opened: asset_server.load("images/levels/day1/scene3_opened_door.png"),
        trapdoor: asset_server.load("images/animations/trapdoor.png"),
        trapdoor_opened: asset_server.load("images/levels/day1/scene2_opened_trapdoor.png"),
    });
}
