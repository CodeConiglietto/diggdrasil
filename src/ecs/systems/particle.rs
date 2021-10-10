use std::convert::TryFrom;

use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::prelude::*;

pub struct ParticleSystem;

impl<'a> System<'a> for ParticleSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, ParticleMapResource>,
        ReadStorage<'a, InputComponent>,
        ReadStorage<'a, PositionComponent>,
        WriteStorage<'a, ParticleComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, mut pmap, inp, pos, mut pac) = data;

        if let Some((_input, player_position)) = (&inp, &pos).join().next() {
            for (eid, pac) in (&eids, &mut pac).join() {
                let (new_pos, new_height) =
                    pac.particle_type.get_new_position(pac.position, pac.height);

                pac.position = new_pos;
                pac.height = new_height;

                pac.particle_type =
                    pac.particle_type
                        .get_new_state(pac.position, pac.height, player_position.pos);

                let delete = match pac.particle_type {
                    ParticleType::Finished => true,
                    _ => {
                        if let Some(layer) = usize::try_from(
                            pac.position.y - player_position.pos.y + MAP_Y_SIZE as i32 / 2,
                        )
                        .ok()
                        .and_then(|y| pmap.contents.get_mut(y))
                        {
                            layer.push(eid);
                            false
                        } else {
                            true
                        }
                    }
                };

                if delete {
                    eids.delete(eid).unwrap();
                }
            }
        }
    }
}
