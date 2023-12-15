use std::fs::File;

use bevy::{
    asset::AssetServer,
    ecs::{bundle::Bundle, system::Res},
    math::Vec3,
    prelude::default,
    sprite::SpriteBundle,
    transform::components::Transform,
};
use serde::Deserialize;

use crate::{
    components::{
        collider::Collider,
        entity_type::{EntityType, PlayerAsset},
    },
    utils::{position::Position, texture::Asset},
};

#[derive(Deserialize)]
struct PlayerData {
    asset_normal: Asset,
    asset_collect: Asset,
    asset_win: Asset,
}

#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    pub(crate) sprite_bundle: SpriteBundle,
    pub(crate) entity_type: EntityType,
    pub(crate) collider: Collider,
}

impl PlayerBundle {
    pub(crate) fn new(position: Position, asset_server: &Res<AssetServer>) -> Self {
        let file = File::open("assets/player/player.json").expect("file should open read only");
        let json: PlayerData = serde_json::from_reader(file).expect("file should be proper JSON");
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(&json.asset_normal.texture),
                transform: Transform {
                    translation: Vec3::new(position.x as f32, position.y as f32, 0.0),
                    scale: Vec3::new(json.asset_normal.scale, json.asset_normal.scale, 1.0),
                    ..default()
                },
                ..default()
            },
            entity_type: EntityType::Player(PlayerAsset::Normal),
            collider: Collider,
        }
    }

    pub(crate) fn change_texture(&mut self, asset_server: &Res<AssetServer>, asset: PlayerAsset) {
        let file = File::open("assets/player/player.json").expect("file should open read only");
        let json: PlayerData = serde_json::from_reader(file).expect("file should be proper JSON");
        match asset {
            PlayerAsset::Normal => {
                self.sprite_bundle.texture = asset_server.load(&json.asset_normal.texture);
            }
            PlayerAsset::Collect => {
                self.sprite_bundle.texture = asset_server.load(&json.asset_collect.texture);
            }
            PlayerAsset::Win => {
                self.sprite_bundle.texture = asset_server.load(&json.asset_win.texture);
            }
        }
    }
}
