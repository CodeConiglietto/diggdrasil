use crate::prelude::*;
use specs::{Join, System, WriteStorage};

pub struct CollisionResolutionSystem;

impl<'a> System<'a> for CollisionResolutionSystem {
    type SystemData = WriteStorage<'a, CollisionComponent>;

    fn run(&mut self, data: Self::SystemData) {
        let mut col = data;

        for col in (&mut col).join() {
            col.tile_collision = None;
            col.entity_collisions.clear();
        }
    }
}
