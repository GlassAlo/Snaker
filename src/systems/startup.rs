use bevy::{
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Res},
};

use crate::resources::{Collectable, Player, Wall};

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Player(asset_server.load_folder("assets/player")));
    commands.insert_resource(Collectable(asset_server.load_folder("assets/collectable")));
    commands.insert_resource(Wall(asset_server.load_folder("assets/wall")));
}
