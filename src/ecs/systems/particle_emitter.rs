use specs::{Builder, Entities, Join, LazyUpdate, Read, ReadStorage, System};

use crate::prelude::*;

pub struct ParticleEmitterSystem;

impl<'a> System<'a> for ParticleEmitterSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, ParticleEmitterComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, lup, pos, pec) = data;

        for (pos, pec) in (&pos, &pec).join() {
            lup.create_entity(&eids)
                .with(ParticleComponent {
                    position: pos.pos,
                    height: 1,
                    particle_type: pec.particle_type,
                })
                .build();
        }
    }
}
