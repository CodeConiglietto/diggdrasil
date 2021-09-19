use crate::prelude::*;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

pub struct MovementResolutionSystem;

impl<'a> System<'a> for MovementResolutionSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, EntityMapResource>,
        ReadStorage<'a, CollisionComponent>,
        WriteStorage<'a, IntendedMovementComponent>,
        WriteStorage<'a, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, mut emap, col, mut imc, mut pos) = data;

        for (eid, col, imc, pos) in (&eids, &col, &mut imc, &mut pos).join() {
            if imc.x_delta != 0 || imc.y_delta != 0 {
                //Take current position
                //Remove entity from entity map at current position
                let ent_vec = &mut emap.contents[[pos.x as usize, pos.y as usize]];

                if !col.tile_collision.is_some() && col.entity_collisions.is_empty() {
                    //Remove entity from its previous position
                    if let Some((index, _item)) =
                        ent_vec.iter().enumerate().find(|(_i, item)| **item == eid)
                    {
                        ent_vec.remove(index);
                    }

                    //Apply intended movement delta
                    pos.x += imc.x_delta;
                    pos.y += imc.y_delta;

                    //Add entity to entity map at new position
                    //If new position is outside the entity map, unload it
                    emap.contents[[pos.x as usize, pos.y as usize]].push(eid);
                }

                imc.x_delta = 0;
                imc.y_delta = 0;
            }
        }
    }
}
