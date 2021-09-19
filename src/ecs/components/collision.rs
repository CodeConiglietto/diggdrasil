use crate::prelude::*;
use specs::{Component, Entity, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct CollisionComponent {
    pub tile_collision: Option<Tile>,
    pub entity_collisions: Vec<Entity>,
}
