use std::fs::File;

use bevy::{
    asset::AssetServer,
    ecs::{bundle::Bundle, system::Res},
    math::{Quat, Vec3},
    prelude::default,
    sprite::SpriteBundle,
    transform::components::Transform,
};
use serde::Deserialize;

use crate::{
    components::{collider::Collider, entity_type::EntityType},
    utils::{direction::Direction, position::Position},
};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WallData {
    asset: String,
    scale: f32,
}

#[derive(Bundle)]
pub(crate) struct WallBundle {
    sprite_bundle: SpriteBundle,
    entity_type: EntityType,
    collider: Collider,
}

impl WallBundle {
    pub(crate) fn new(
        position: Position,
        direction: Direction,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let file = File::open("assets/wall/wall.json").expect("file should open read only");
        let json: WallData = serde_json::from_reader(file).expect("file should be proper JSON");
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(&json.asset),
                transform: Transform {
                    translation: Vec3::new(position.x as f32, position.y as f32, 0.0),
                    rotation: Quat::from_rotation_z(direction.rotation_angle().to_radians()),
                    scale: Vec3::new(json.scale, json.scale, 0.2),
                    ..default()
                },
                ..default()
            },
            entity_type: EntityType::Wall,
            collider: Collider,
        }
    }
}
