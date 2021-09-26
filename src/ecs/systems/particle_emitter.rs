use std::convert::TryFrom;

use specs::{Entities, Join, System, Read, ReadStorage};

use crate::prelude::*;

pub struct ParticleSystem;

impl<'a> System<'a> for ParticleEmitterSystem {
    type SystemData = (
        Read<'a, LazyUpdate>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, ParticleEmitterComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pos, pec) = data;

        for (pos, pec) in (&pos, &pec).join() {
            let PositionComponent{x, y} = pos;

            lup.create_entity(&eids)
                .with(ParticleComponent {
                    position: (
                        x,
                        y, 
                        1,
                    ),
                    particle_type: pec.particle_type,
                })
                .build();
        }
    }
}