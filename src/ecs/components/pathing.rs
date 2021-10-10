use std::{convert::TryFrom, ops::Not};

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
        start: IPosition,
        end: IPosition,
    ) -> Option<Vec<IPosition>> {
        let loaded_offset = tile_world.offset * CHUNK_SIZE as i32;

        let start_loaded = start - loaded_offset;
        let end_loaded = end - loaded_offset;

        let range = 0..(CHUNK_SIZE as i32 * 3);

        if !range.contains(&start_loaded.x)
            || !range.contains(&start_loaded.y)
            || !range.contains(&end_loaded.x)
            || !range.contains(&end_loaded.y)
        {
            return None;
        }

        let buffer = &tile_world.buffer;

        self.a_star
            .a_star_simple(
                UPosition::try_from(start_loaded).unwrap(),
                UPosition::try_from(end_loaded).unwrap(),
                |prev_loaded, loaded| {
                    let diff = IPosition::try_from(loaded).unwrap()
                        - IPosition::try_from(prev_loaded).unwrap();

                    let buffer_pos = loaded / u32::try_from(CHUNK_SIZE).unwrap();
                    let local_pos = loaded % u32::try_from(CHUNK_SIZE).unwrap();

                    buffer[TileWorldResource::buffer_idx(buffer_pos).unwrap()].tiles
                        [local_pos.to_idx().unwrap()]
                    .tile
                    .tile_type
                    .collides()
                    .not()
                    .then(|| 1 + diff.x.abs() as u32 + diff.y.abs() as u32)
                },
            )
            .map(|path| {
                let path: Vec<_> = path
                    .map(|loaded| IPosition::try_from(loaded).unwrap() + loaded_offset)
                    .collect();

                path
            })
    }
}

impl Default for PathingComponent {
    fn default() -> Self {
        Self::new()
    }
}
