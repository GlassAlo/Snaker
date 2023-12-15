#![warn(clippy::all)]

use bevy::prelude::*;
use resources::{Collectable, Player, Score, Wall};
use systems::map;

mod bundles;
mod components;
mod resources;
mod systems;
mod utils;

#[derive(Event, Default)]
struct CollisionEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, map::spawn_walls))
        .insert_resource(Score(0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Player(asset_server.load_folder("assets/player")));
    commands.insert_resource(Collectable(asset_server.load_folder("assets/collectable")));
    commands.insert_resource(Wall(asset_server.load_folder("assets/wall")));
}
