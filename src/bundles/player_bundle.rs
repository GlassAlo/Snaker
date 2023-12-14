use bevy::{
    asset::AssetServer,
    ecs::{bundle::Bundle, system::Res},
    math::Vec3,
    prelude::default,
    sprite::SpriteBundle,
    transform::components::Transform,
};

use crate::{
    components::{collider::Collider, entity_type::EntityType},
    utils::position::Position,
};

#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    pub(crate) sprite_bundle: SpriteBundle,
    pub(crate) entity_type: EntityType,
    pub(crate) collider: Collider,
}

impl PlayerBundle {
    pub(crate) fn new(position: Position, asset_server: &Res<AssetServer>) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("player/Faker_Head.png"),
                transform: Transform {
                    translation: Vec3::new(position.x as f32, position.y as f32, 0.0),
                    scale: Vec3::new(0.1, 0.1, 0.2),
                    ..default()
                },
                ..default()
            },
            entity_type: EntityType::Player,
            collider: Collider,
        }
    }
}
