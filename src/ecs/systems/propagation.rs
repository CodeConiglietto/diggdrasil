use rand::prelude::*;
use specs::prelude::*;

use crate::prelude::*;

pub struct PropagationSystem;

impl<'a> System<'a> for PropagationSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        WriteExpect<'a, TileWorldResource>,
        ReadStorage<'a, VegPropagationComponent>,
        WriteStorage<'a, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, lup, mut  twld, vpc, mut pos) = data;

        for (eid, vpc) in (&eids, &vpc).join() {
            if let Some(this_pos) = pos.get(eid) {
                if thread_rng().gen_range(0..vpc.propagation_chance) == 0 {
                    let target_pos = this_pos.pos + IPosition::new(thread_rng().gen_range(-1..=1), thread_rng().gen_range(-1..=1));

                    if let Some(tile) = twld.get_mut(target_pos) {
                        if tile.entities.is_empty() && !tile.tile.tile_type.collides() {
                            twld.spawn_entity(
                                vpc.parent_builder
                                    .build(
                                        &lup,
                                        &eids,
                                    ),
                                target_pos, 
                                &mut pos,
                            );
                        } else {
                            println!("Entity attempting to propagate into unsuitable tile!");
                        }
                    } else {
                        println!("Entity attempting to propagate into an unloaded tile!");
                    }
                }
            } else {
                println!("Entity attempting to propagate without position!");
            }
        }
    }
}
