use std::ops::Not;

use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct PathingComponent {
    pub a_star: AStar,
}

impl PathingComponent {
    pub fn new() -> Self {
        Self {
            a_star: AStar::new(CHUNK_SIZE * 3, CHUNK_SIZE * 3),
        }
    }

    ///Paths are returned backwards and should be treated as a stack
    pub fn pathfind(
        &mut self,
        tile_world: &TileWorldResource,
        start: (i32, i32),
        end: (i32, i32),
    ) -> Option<Vec<(i32, i32)>> {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;

        let (offset_x, offset_y) = tile_world.offset;
        let (loaded_offset_x, loaded_offset_y) =
            (offset_x * CHUNK_SIZE as i32, offset_y * CHUNK_SIZE as i32);

        let (start_loaded_x, start_loaded_y) =
            (start_x - loaded_offset_x, start_y - loaded_offset_y);
        let (end_loaded_x, end_loaded_y) = (end_x - loaded_offset_x, end_y - loaded_offset_y);

        let range = 0..(CHUNK_SIZE as i32 * 3);

        if !range.contains(&start_loaded_x)
            || !range.contains(&start_loaded_y)
            || !range.contains(&end_loaded_x)
            || !range.contains(&end_loaded_y)
        {
            return None;
        }

        let buffer = &tile_world.buffer;

        self.a_star
            .a_star_simple(
                (start_loaded_x as usize, start_loaded_y as usize),
                (end_loaded_x as usize, end_loaded_y as usize),
                |(prev_loaded_x, prev_loaded_y), (loaded_x, loaded_y)| {
                    let diff_x = loaded_x as i32 - prev_loaded_x as i32;
                    let diff_y = loaded_y as i32 - prev_loaded_y as i32;

                    let (buffer_x, buffer_y) = (loaded_x / CHUNK_SIZE, loaded_y / CHUNK_SIZE);
                    let (local_x, local_y) = (loaded_x % CHUNK_SIZE, loaded_y % CHUNK_SIZE);

                    buffer[TileWorldResource::buffer_idx(buffer_x, buffer_y)].tiles
                        [[local_x, local_y]]
                    .tile
                    .tile_type
                    .collides()
                    .not()
                    .then(|| 1 + diff_x.abs() as u32 + diff_y.abs() as u32)
                },
            )
            .map(|path| {
                let path: Vec<_> = path
                    .map(|(loaded_x, loaded_y)| {
                        (
                            loaded_x as i32 + loaded_offset_x,
                            loaded_y as i32 + loaded_offset_y,
                        )
                    })
                    .collect();

                path
            })
    }
}
