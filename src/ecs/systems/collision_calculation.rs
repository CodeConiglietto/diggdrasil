use log::{debug, warn};
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

use crate::prelude::*;

pub struct CollisionCalculationSystem;

impl<'a> System<'a> for CollisionCalculationSystem {
    type SystemData = (
        ReadExpect<'a, TileWorldResource>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, IntendedMovementComponent>,
        ReadStorage<'a, ColliderComponent>,
        WriteStorage<'a, CollisionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (twld, pos, imc, coc, mut col) = data;

        for (pos, imc, col) in (&pos, &imc, &mut col).join() {
            if imc.x_delta != 0 || imc.y_delta != 0 {
                let (new_x, new_y) = (pos.x + imc.x_delta, pos.y + imc.y_delta);

                if let Some(chunk_tile) = twld.get((new_x, new_y)) {
                    if chunk_tile.tile.tile_type.collides() {
                        col.tile_collision = Some(chunk_tile.tile);
                    }

                    for entity in &chunk_tile.entities {
                        if coc.get(*entity).is_some() {
                            col.entity_collisions.push(*entity);
                            break;
                        }
                    }

                    debug!("Colliding with tile? {}", col.tile_collision.is_some());
                    debug!("Colliding with {} entities", col.entity_collisions.len());
                } else {
                    warn!("Entity colliding with unloaded tile");
                }
            }
        }
    }
}
