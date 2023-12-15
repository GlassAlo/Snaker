#![warn(clippy::all)]

use bevy::prelude::*;
use resources::Score;
use systems::{map, startup::setup};

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
