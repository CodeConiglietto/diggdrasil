use specs::{Builder, Entities, Join, LazyUpdate, ReadExpect, System, WriteExpect, WriteStorage};

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

    //This makes black magic
    fn run(&mut self, data: Self::SystemData) {
        let (eids, lup, mut twld, mut hpc, mut pos, mut dec) = data;

        for (eid, hpc, dec) in (&eids, &mut hpc, &mut dec).join() {
            let PositionComponent { x, y } = *(pos.get(eid).unwrap());

            for _ in 0..hpc.turn_damage {
                if let Some(particle) = hpc.hit_particle {
                    lup.create_entity(&eids)
                        .with(ParticleComponent {
                            position: (
                                //TODO: make these values more sane
                                x, // + thread_rng().gen_range(-1..=1) as i32,
                                y, // + thread_rng().gen_range(-1..=1) as i32,
                                4, //thread_rng().gen_range(1..5),
                            ),
                            particle_type: particle,
                        })
                        .build();
                }

                hpc.value -= 1;

                if hpc.value == 0 {
                    for dec in dec.contained_entities.drain(..) {
                        twld.spawn_entity(dec, (x, y), &mut pos);
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
