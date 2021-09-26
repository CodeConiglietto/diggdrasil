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
            let (player_x, player_y) = (player_position.x, player_position.y);

            for (eid, pac) in (&eids, &mut pac).join() {
                let (x, y, z) = pac.position;

                pac.position = pac.particle_type.get_new_position((x, y, z));
                pac.particle_type = pac
                    .particle_type
                    .get_new_state((x, y, z), (player_x, player_y));

                let (_x, y, _z) = pac.position;

                let delete = match pac.particle_type {
                    ParticleType::Finished => true,

                    _ => {
                        if let Some(layer) = usize::try_from(y - player_y + MAP_Y_SIZE as i32 / 2)
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
