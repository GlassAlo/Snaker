use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

use crate::{
    bundles::wall_bundle::WallBundle,
    utils::{direction::Direction, position::Position},
};

pub(crate) fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    let wall = WallBundle::new(
        Position {
            x: 0.0 as f32,
            y: 0.0,
        },
        Direction::Up,
        &asset_server,
    );
    let size = wall.data.asset.get_size();
    for x in 0..=10 {
        let wall = WallBundle::new(
            Position {
                x: x as f32 * size.width,
                y: 0.0,
            },
            Direction::Up,
            &asset_server,
        );
        commands.spawn(wall);
    }
}
