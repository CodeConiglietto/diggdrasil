use log::{debug, trace};
use specs::{Entity, WriteStorage};

use crate::prelude::*;

pub struct TileWorldResource {
    offset: (i32, i32),
    buffer: [Chunk; 9],
}

impl TileWorldResource {
    pub fn new(gen_data: &mut GenData) -> Self {
        let (offset_x, offset_y) = (-1i32, -1i32);
        let mut buffer = [
            Chunk::default(),
            Chunk::default(),
            Chunk::default(),
            Chunk::default(),
            Chunk::default(),
            Chunk::default(),
            Chunk::default(),
            Chunk::default(),
            Chunk::default(),
        ];

        for buffer_x in 0..3usize {
            for buffer_y in 0..3usize {
                let (chunk_x, chunk_y) = (buffer_x as i32 + offset_x, buffer_y as i32 + offset_y);
                buffer[buffer_idx(buffer_x, buffer_y)].generate((chunk_x, chunk_y), gen_data);
            }
        }

        let mut world = Self {
            offset: (offset_x, offset_y),
            buffer,
        };

        let (left, top) = local_to_global_position((offset_x, offset_y), (0, 0));
        let (right, bottom) = local_to_global_position((offset_x + 3, offset_y + 3), (0, 0));

        for x in left..right {
            for y in top..bottom {
                world.refresh_tile_variant((x, y))
            }
        }

        world
    }

    pub fn offset(&self) -> (i32, i32) {
        self.offset
    }

    pub fn get(&self, pos: (i32, i32)) -> Option<&ChunkTile> {
        let ((chunk_x, chunk_y), (local_x, local_y)) = global_to_local_position(pos);
        let (offset_x, offset_y) = self.offset;

        let (buffer_x, buffer_y) = ((chunk_x - offset_x), (chunk_y - offset_y));

        let loaded = (0..3).contains(&buffer_x) && (0..3).contains(&buffer_y);

        loaded.then(|| {
            &self.buffer[buffer_idx(buffer_x as usize, buffer_y as usize)].tiles[[local_x, local_y]]
        })
    }

    pub fn get_mut(&mut self, pos: (i32, i32)) -> Option<&mut ChunkTile> {
        let ((chunk_x, chunk_y), (local_x, local_y)) = global_to_local_position(pos);
        let (offset_x, offset_y) = self.offset;

        let (buffer_x, buffer_y) = ((chunk_x - offset_x), (chunk_y - offset_y));

        let loaded = (0..3).contains(&buffer_x) && (0..3).contains(&buffer_y);

        loaded.then(move || {
            &mut self.buffer[buffer_idx(buffer_x as usize, buffer_y as usize)].tiles
                [[local_x, local_y]]
        })
    }

    pub fn refresh_tile_variant(&mut self, pos: (i32, i32)) {
        let neighbours = self.get_neighbours(pos);

        if let Some(chunk_tile) = self.get_mut(pos) {
            chunk_tile.tile.tile_variant = TileVariant::get_from_neighbours(neighbours);
        } else {
            println!("Cannot refresh {:?}", pos);
        }
    }

    pub fn refresh_tile_and_adjacent_variants(&mut self, (x, y): (i32, i32)) {
        self.refresh_tile_variant((x, y - 1));
        self.refresh_tile_variant((x - 1, y));
        self.refresh_tile_variant((x, y));
        self.refresh_tile_variant((x + 1, y));
        self.refresh_tile_variant((x, y + 1));
    }

    //Get neighbour tile types in (u, d, l, r) order
    pub fn get_neighbours(
        &self,
        (x, y): (i32, i32),
    ) -> (
        Option<TileType>,
        Option<TileType>,
        Option<TileType>,
        Option<TileType>,
    ) {
        (
            self.get((x, y - 1)).map(|tile| tile.tile.tile_type),
            self.get((x, y + 1)).map(|tile| tile.tile.tile_type),
            self.get((x - 1, y)).map(|tile| tile.tile.tile_type),
            self.get((x + 1, y)).map(|tile| tile.tile.tile_type),
        )
    }

    pub fn update_center(&mut self, center_pos: (i32, i32), gen_data: &mut GenData) {
        let (offset_x, offset_y) = self.offset;

        let ((center_chunk_x, center_chunk_y), _) = global_to_local_position(center_pos);
        let (new_offset_x, new_offset_y) = (center_chunk_x - 1, center_chunk_y - 1);
        let (offset_diff_x, offset_diff_y) = (new_offset_x - offset_x, new_offset_y - offset_y);

        if (offset_x, offset_y) != (new_offset_x, new_offset_y) {
            // Relocate chunks
            for x in 0..3i32 {
                let buffer_x = if offset_diff_x > 0 { x } else { 2 - x };

                for y in 0..3i32 {
                    let buffer_y = if offset_diff_y > 0 { y } else { 2 - y };

                    let (chunk_x, chunk_y) =
                        (buffer_x as i32 + offset_x, buffer_y as i32 + offset_y);

                    let (new_chunk_x, new_chunk_y) = (
                        buffer_x as i32 + new_offset_x,
                        buffer_y as i32 + new_offset_y,
                    );

                    let (from_buffer_x, from_buffer_y) =
                        (buffer_x + offset_diff_x, buffer_y + offset_diff_y);
                    let (to_buffer_x, to_buffer_y) =
                        (buffer_x - offset_diff_x, buffer_y - offset_diff_y);

                    let unload = !(0..3).contains(&to_buffer_x) || !(0..3).contains(&to_buffer_y);
                    let generate =
                        !(0..3).contains(&from_buffer_x) || !(0..3).contains(&from_buffer_y);

                    if unload {
                        debug!(
                            "Unloading chunk ({},{}) from buffer index ({},{})",
                            chunk_x, chunk_y, buffer_x, buffer_y
                        );

                        // TODO Unload chunk here

                        self.buffer[buffer_idx(buffer_x as usize, buffer_y as usize)]
                            .unload(gen_data);
                    }

                    if generate {
                        debug!(
                            "Generating chunk ({},{}) at buffer index ({},{})",
                            new_chunk_x, new_chunk_y, buffer_x, buffer_y
                        );

                        self.buffer[buffer_idx(buffer_x as usize, buffer_y as usize)]
                            .generate((new_chunk_x, new_chunk_y), gen_data);
                    } else {
                        trace!(
                            "Moving chunk ({},{}) from buffer index ({},{}) to ({}, {})",
                            new_chunk_x,
                            new_chunk_y,
                            from_buffer_x,
                            from_buffer_y,
                            buffer_x,
                            buffer_y,
                        );
                        self.buffer.swap(
                            buffer_idx(buffer_x as usize, buffer_y as usize),
                            buffer_idx(from_buffer_x as usize, from_buffer_y as usize),
                        );
                    }
                }
            }

            self.offset = (new_offset_x, new_offset_y);

            // TODO Optimize this to only recompute variants on new chunks and tiles adjacent to them rather than everywhere

            let (left, top) = local_to_global_position((new_offset_x, new_offset_y), (0, 0));
            let (right, bottom) =
                local_to_global_position((new_offset_x + 3, new_offset_y + 3), (0, 0));

            for x in left..right {
                for y in top..bottom {
                    self.refresh_tile_variant((x, y));
                }
            }
        }
    }

    pub fn spawn_entity(
        &mut self,
        entity: Entity,
        (x, y): (i32, i32),
        position_component: &mut WriteStorage<PositionComponent>,
    ) {
        assert!(
            position_component
                .insert(entity, PositionComponent { x, y })
                .unwrap()
                .is_none(),
            "Cannot spawn entity that already has a position!"
        );

        self.get_mut((x, y)).unwrap().entities.push(entity);
    }

    pub fn despawn_entity(
        &mut self,
        entity: Entity,
        position_component: &mut WriteStorage<PositionComponent>,
    ) {
        let pos = position_component.remove(entity).unwrap();
        let entities = &mut self.get_mut((pos.x, pos.y)).unwrap().entities;

        let (index, _item) = entities
            .iter()
            .enumerate()
            .find(|(_i, item)| **item == entity)
            .unwrap();
        entities.remove(index);
    }
}

fn buffer_idx(buffer_x: usize, buffer_y: usize) -> usize {
    buffer_y * 3 + buffer_x
}
