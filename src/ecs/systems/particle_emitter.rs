use std::convert::TryFrom;

use specs::{Builder, Entities, Join, LazyUpdate, System, Read, ReadStorage};

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
            let PositionComponent{x, y} = pos;

            lup.create_entity(&eids)
                .with(ParticleComponent {
                    position: (
                        *x,
                        *y, 
                        1,
                    ),
                    particle_type: pec.particle_type,
                })
                .build();
        }
    }
}