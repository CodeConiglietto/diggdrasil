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
            if imc.delta != IPosition::ZERO {
                let new_pos = pos.pos + imc.delta;

                if let Some(chunk_tile) = twld.get(new_pos) {
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
