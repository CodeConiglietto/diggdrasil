use specs::{Builder, Entities, Join, LazyUpdate, ReadExpect, System, WriteExpect, WriteStorage};

use crate::prelude::*;

pub struct PerceptionResolutionSystem;

impl<'a> System<'a> for PerceptionResolutionSystem {
    type SystemData = (
        WriteStorage<'a, AIPerceptionComponent>,
        WriteStorage<'a, AIGoalComponent>,
    );

    //This makes black magic
    fn run(&mut self, data: Self::SystemData) {
        let (mut per, mut gol) = data;

        for (per, gol) in (&mut per, &mut gol).join() {


            per.all.clear();
        }
    }
}
