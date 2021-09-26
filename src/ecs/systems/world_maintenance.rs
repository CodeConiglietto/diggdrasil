use specs::{Join, ReadStorage, System, WriteExpect};

use crate::prelude::*;

pub struct WorldMaintenanceSystem;

impl<'a> System<'a> for WorldMaintenanceSystem {
    type SystemData = (
        WriteExpect<'a, TileWorldResource>,
        ReadStorage<'a, InputComponent>,
        WorldData<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut twld, input, mut world_data) = data;

        if let Some((_input, position)) = (&input, &world_data.position).join().next() {
            twld.update_center((position.x, position.y), &mut world_data)
        }
    }
}
