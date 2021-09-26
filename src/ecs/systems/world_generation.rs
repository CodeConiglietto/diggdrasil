use specs::{Join, ReadStorage, System, WriteExpect};

use crate::prelude::*;

pub struct WorldGenerationSystem;

impl<'a> System<'a> for WorldGenerationSystem {
    type SystemData = (
        WriteExpect<'a, TileWorldResource>,
        ReadStorage<'a, InputComponent>,
        GenData<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut twld, input, mut gen_data) = data;

        if let Some((_input, position)) = (&input, &gen_data.position).join().next() {
            twld.update_center((position.x, position.y), &mut gen_data)
        }
    }
}
