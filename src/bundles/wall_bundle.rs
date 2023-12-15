use std::fs::File;

use bevy::{
    asset::AssetServer,
    ecs::{bundle::Bundle, component::Component, system::Res},
    math::{Quat, Vec3},
    prelude::default,
    sprite::SpriteBundle,
    transform::components::Transform,
};
use serde::Deserialize;

use crate::{
    components::{collider::Collider, entity_type::EntityType},
    utils::{direction::Direction, position::Position, texture::Asset},
};

#[derive(Deserialize, Component)]
pub(crate) struct WallData {
    pub(crate) asset: Asset,
}

#[derive(Bundle)]
pub(crate) struct WallBundle {
    sprite_bundle: SpriteBundle,
    entity_type: EntityType,
    collider: Collider,
    pub(crate) data: WallData,
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
                texture: asset_server.load(&json.asset.texture),
                transform: Transform {
                    translation: Vec3::new(position.x as f32, position.y as f32, 0.0),
                    rotation: Quat::from_rotation_z(direction.rotation_angle().to_radians()),
                    scale: Vec3::new(json.asset.scale, json.asset.scale, json.asset.scale),
                    ..default()
                },
                ..default()
            },
            entity_type: EntityType::Wall,
            collider: Collider,
            data: json,
        }
    }
}
