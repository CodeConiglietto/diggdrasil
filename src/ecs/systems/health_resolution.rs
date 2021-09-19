use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

use crate::prelude::*;

pub struct HealthResolutionSystem;

impl<'a> System<'a> for HealthResolutionSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, EntityMapResource>,
        ReadStorage<'a, HealthComponent>,
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, DeathComponent>,
    );

    //This makes black magic
    fn run(&mut self, data: Self::SystemData) {
        let (eids, mut emap, hpc, mut pos, mut dec) = data;

        for (eid, hpc, dec) in (&eids, &hpc, &mut dec).join() {
            if hpc.value == 0 {
                let PositionComponent { x, y } = *(pos.get(eid).unwrap());

                for dec in dec.contained_entities.drain(..) {
                    emap.spawn_entity(dec, (x, y), &mut pos);
                }

                emap.despawn_entity(eid, &mut pos);
                eids.delete(eid).unwrap();
            }
        }
    }
}
