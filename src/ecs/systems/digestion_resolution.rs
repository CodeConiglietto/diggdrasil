use rand::prelude::*;
use specs::{Entities, Join, System, WriteStorage};

use crate::prelude::*;

pub struct DigestionResolutionSystem;

impl<'a> System<'a> for DigestionResolutionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, DigestionComponent>,
        WriteStorage<'a, EdibleComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, mut hpc, mut dig, mut edc) = data;

        for (hpc, dig) in (&mut hpc, &mut dig).join() {
            if thread_rng().gen::<bool> () {
                if let Some(edible) = dig.contents.first() {
                    let mut nutrient = edc.get_mut(*edible).unwrap();

                    nutrient.nutrient_value -= 1;

                    //TODO: assert that nutrients have no position, and that they're not in the map
                    if nutrient.nutrient_value == 0 {
                        eids.delete(*edible).unwrap();
                        dig.contents.remove(0);
                    }

                    //TODO: change HP to use a
                    if hpc.value < hpc.max_value {
                        hpc.value += 1;
                    }
                } else {
                    hpc.turn_damage += 1;
                }
            }
        }
    }
}
