use std::ops::Not;

use specs::{Entity, WriteStorage};

use crate::prelude::*;

pub struct TileWorldResource {
    pub offset: (i32, i32),
    pub buffer: [Chunk; 9],
    pub a_star: AStar,
}

impl TileWorldResource {
    pub fn new(gen_package: &GenPackageResource, world_data: &mut WorldData) -> Self {
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
                buffer[Self::buffer_idx(buffer_x, buffer_y)].generate(
                    (chunk_x, chunk_y),
                    gen_package,
                    world_data,
                );
            }
        }

        let mut world = Self {
            offset: (offset_x, offset_y),
            buffer,
            a_star: AStar::new(CHUNK_SIZE * 3, CHUNK_SIZE * 3),
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

    pub fn get(&self, pos: (i32, i32)) -> Option<&ChunkTile> {
        let ((chunk_x, chunk_y), (local_x, local_y)) = global_to_local_position(pos);
        let (offset_x, offset_y) = self.offset;

        let (buffer_x, buffer_y) = ((chunk_x - offset_x), (chunk_y - offset_y));

        let loaded = (0..3).contains(&buffer_x) && (0..3).contains(&buffer_y);

        loaded.then(|| {
            &self.buffer[Self::buffer_idx(buffer_x as usize, buffer_y as usize)].tiles
                [[local_x, local_y]]
        })
    }

    pub fn get_mut(&mut self, pos: (i32, i32)) -> Option<&mut ChunkTile> {
        let ((chunk_x, chunk_y), (local_x, local_y)) = global_to_local_position(pos);
        let (offset_x, offset_y) = self.offset;

        let (buffer_x, buffer_y) = ((chunk_x - offset_x), (chunk_y - offset_y));

        let loaded = (0..3).contains(&buffer_x) && (0..3).contains(&buffer_y);

        loaded.then(move || {
            &mut self.buffer[Self::buffer_idx(buffer_x as usize, buffer_y as usize)].tiles
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

    pub fn buffer_idx(buffer_x: usize, buffer_y: usize) -> usize {
        buffer_idx(buffer_x, buffer_y)
    }

    pub fn pathfind(&mut self, start: (i32, i32), end: (i32, i32)) -> Option<Vec<(i32, i32)>> {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;

        let (offset_x, offset_y) = self.offset;
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

        let buffer = &self.buffer;

        self.a_star
            .a_star_simple(
                (start_loaded_x as usize, start_loaded_y as usize),
                (end_loaded_x as usize, end_loaded_y as usize),
                |loaded_x, loaded_y| {
                    let (buffer_x, buffer_y) = (loaded_x / CHUNK_SIZE, loaded_y / CHUNK_SIZE);
                    let (local_x, local_y) = (loaded_x % CHUNK_SIZE, loaded_y % CHUNK_SIZE);

                    buffer[buffer_idx(buffer_x, buffer_y)].tiles[[local_x, local_y]]
                        .tile
                        .tile_type
                        .collides()
                        .not()
                        .then(|| 1)
                },
            )
            .map(|path| {
                let mut path: Vec<_> = path
                    .map(|(loaded_x, loaded_y)| {
                        (
                            loaded_x as i32 + loaded_offset_x,
                            loaded_y as i32 + loaded_offset_y,
                        )
                    })
                    .collect();

                path.reverse();

                path
            })
    }
}

fn buffer_idx(buffer_x: usize, buffer_y: usize) -> usize {
    buffer_y * 3 + buffer_x
}
