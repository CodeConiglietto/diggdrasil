use crate::prelude::*;
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

pub struct MovementResolutionSystem;

impl<'a> System<'a> for MovementResolutionSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, TileWorldResource>,
        ReadStorage<'a, CollisionComponent>,
        WriteStorage<'a, IntendedMovementComponent>,
        WriteStorage<'a, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, mut twld, col, mut imc, mut pos) = data;

        for (eid, col, imc, pos) in (&eids, &col, &mut imc, &mut pos).join() {
            if imc.x_delta != 0 || imc.y_delta != 0 {
                //Take current position
                //Remove entity from entity map at current position

                let new_x = pos.x + imc.x_delta;
                let new_y = pos.y + imc.y_delta;

                if let Some(new_chunk_tile) = twld.get_mut((new_x, new_y)) {
                    if !col.tile_collision.is_some() && col.entity_collisions.is_empty() {
                        //Apply intended movement delta

                        //Add entity to entity map at new position
                        //If new position is outside the entity map, unload it
                        new_chunk_tile.entities.push(eid);

                        let chunk_tile = twld.get_mut((pos.x, pos.y)).unwrap();
                        //Remove entity from its previous position
                        let index = chunk_tile
                            .entities
                            .iter()
                            .enumerate()
                            .find(|(_i, item)| **item == eid)
                            .unwrap()
                            .0;

                        chunk_tile.entities.remove(index);

                        pos.x = new_x;
                        pos.y = new_y;
                    }
                } else {
                    println!("Entity trying to move into unloaded tile!");
                }

                imc.x_delta = 0;
                imc.y_delta = 0;
            }
        }
    }
}
