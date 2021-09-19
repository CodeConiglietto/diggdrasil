use std::convert::TryFrom;

use specs::{Entities, Join, System, Write, WriteStorage};

use crate::prelude::*;

pub struct ParticleSystem;

impl<'a> System<'a> for ParticleSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, ParticleMapResource>,
        WriteStorage<'a, ParticleComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, mut pmap, mut pac) = data;

        for (eid, pac) in (&eids, &mut pac).join() {
            let (x, y, z) = pac.position;

            pac.position = pac.particle_type.get_new_position((x, y, z));
            pac.particle_type = pac.particle_type.get_new_state((x, y, z));

            let (_x, y, _z) = pac.position;

            match pac.particle_type {
                ParticleType::Finished => eids.delete(eid).unwrap(),
                _ => pmap.contents[usize::try_from(y).unwrap()].push(eid),
            }
        }
    }
}
