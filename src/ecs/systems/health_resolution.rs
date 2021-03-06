use specs::{Entities, Join, LazyUpdate, ReadExpect, System, WriteExpect, WriteStorage};

use crate::prelude::*;

pub struct HealthResolutionSystem;

impl<'a> System<'a> for HealthResolutionSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, LazyUpdate>,
        WriteExpect<'a, TileWorldResource>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, DeathComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, lup, mut twld, mut hpc, mut pos, mut dec) = data;

        for (eid, hpc, dec) in (&eids, &mut hpc, &mut dec).join() {
            let position = pos.get(eid).unwrap().pos;

            for _ in 0..hpc.turn_damage {
                if let Some(particle) = hpc.hit_particle {
                    particle.build(&lup, &eids, position);
                }

                hpc.value -= 1;

                println!("New health: {}", hpc.value);

                if hpc.value == 0 {
                    for dec in dec.contained_entities.drain(..) {
                        twld.spawn_entity(dec, position, &mut pos);
                    }

                    twld.despawn_entity(eid, &mut pos);
                    eids.delete(eid).unwrap();
                    break;
                }
            }

            hpc.turn_damage = 0;
        }
    }
}
