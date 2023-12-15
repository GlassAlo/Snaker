use bevy::ecs::component::Component;

pub(crate) enum PlayerAsset {
    Normal,
    Collect,
    Win,
}

#[derive(Component)]
pub(crate) enum EntityType {
    Player(PlayerAsset),
    Collectable,
    Wall,
}
