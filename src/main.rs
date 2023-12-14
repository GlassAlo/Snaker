#![warn(clippy::all)]

use bevy::prelude::*;
use bundles::{player_bundle::PlayerBundle, wall_bundle::WallBundle};
use components::{collider::Collider, entity_type::EntityType};
use resources::{Collectable, Player, Score, Wall};
use utils::{direction::Direction, position::Position};

mod bundles;
mod components;
mod resources;
mod utils;

#[derive(Event, Default)]
struct CollisionEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_walls))
        .insert_resource(Score(0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Player(asset_server.load_folder("assets/player")));
    commands.insert_resource(Collectable(asset_server.load_folder("assets/collectable")));
    commands.insert_resource(Wall(asset_server.load_folder("assets/wall")));
}

fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(WallBundle::new(
        Position { x: 1.0, y: 1.0 },
        Direction::Down,
        &asset_server,
    ));
    commands.spawn(PlayerBundle::new(
        Position { x: 10.0, y: 10.0 },
        &asset_server,
    ));
}
