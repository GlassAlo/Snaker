use bevy::{
    asset::{Handle, LoadedFolder},
    ecs::system::Resource,
};

#[derive(Resource)]
pub(crate) struct Score(pub(crate) i32);

#[derive(Resource)]
pub(crate) struct Player(pub(crate) Handle<LoadedFolder>);

#[derive(Resource)]
pub(crate) struct Collectable(pub(crate) Handle<LoadedFolder>);

#[derive(Resource)]
pub(crate) struct Wall(pub(crate) Handle<LoadedFolder>);
