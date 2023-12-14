use bevy::ecs::component::Component;

#[derive(Component)]
pub(crate) enum EntityType {
    Player,
    Collectable,
    Wall,
}
