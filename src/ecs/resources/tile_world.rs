use std::convert::TryFrom;

use specs::{Entity, WriteStorage};

use crate::prelude::*;

pub struct TileWorldResource {
    pub offset: IPosition,
    pub buffer: [Chunk; 9],
}

impl TileWorldResource {
    pub fn new(gen_package: &GenPackageResource, world_data: &mut WorldData) -> Self {
        let offset = IPosition::new(-1, -1);
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

        for buffer_x in 0..3 {
            for buffer_y in 0..3 {
                let buffer_pos = UPosition::new(buffer_x, buffer_y);
                let chunk_pos = IPosition::try_from(buffer_pos).unwrap() + offset;

                buffer[Self::buffer_idx(buffer_pos).unwrap()].generate(
                    chunk_pos,
                    gen_package,
                    world_data,
                );
            }
        }

        let mut world = Self { offset, buffer };

        let top_left = IPosition::global_from_local(offset, UPosition::ZERO);
        let bottom_right =
            IPosition::global_from_local(offset + IPosition::new(3, 3), UPosition::ZERO);

        for x in top_left.x..bottom_right.x {
            for y in top_left.y..bottom_right.y {
                world.refresh_tile_variant(IPosition::new(x, y))
            }
        }

        world
    }

    pub fn get(&self, pos: IPosition) -> Option<&ChunkTile> {
        let (chunk_pos, local_pos) = pos.global_to_local();
        let buffer_pos = chunk_pos - self.offset;

        let loaded = (0..3).contains(&buffer_pos.x) && (0..3).contains(&buffer_pos.y);

        loaded.then(|| {
            &self.buffer[Self::buffer_idx(UPosition::try_from(buffer_pos).unwrap()).unwrap()].tiles
                [local_pos.to_idx().unwrap()]
        })
    }

    pub fn get_mut(&mut self, pos: IPosition) -> Option<&mut ChunkTile> {
        let (chunk_pos, local_pos) = pos.global_to_local();
        let buffer_pos = chunk_pos - self.offset;

        let loaded = (0..3).contains(&buffer_pos.x) && (0..3).contains(&buffer_pos.y);

        loaded.then(move || {
            &mut self.buffer[Self::buffer_idx(UPosition::try_from(buffer_pos).unwrap()).unwrap()]
                .tiles[local_pos.to_idx().unwrap()]
        })
    }

    pub fn refresh_tile_variant(&mut self, pos: IPosition) {
        let neighbours = self.get_neighbours(pos);

        if let Some(chunk_tile) = self.get_mut(pos) {
            chunk_tile.tile.tile_variant = TileVariant::get_from_neighbours(neighbours);
        } else {
            println!("Cannot refresh {:?}", pos);
        }
    }

    pub fn refresh_tile_and_adjacent_variants(&mut self, pos: IPosition) {
        self.refresh_tile_variant(pos.up());
        self.refresh_tile_variant(pos.left());
        self.refresh_tile_variant(pos);
        self.refresh_tile_variant(pos.right());
        self.refresh_tile_variant(pos.down());
    }

    //Get neighbour tile types in (u, d, l, r) order
    pub fn get_neighbours(&self, pos: IPosition) -> [Option<TileType>; 4] {
        [
            self.get(pos.up()).map(|tile| tile.tile.tile_type),
            self.get(pos.down()).map(|tile| tile.tile.tile_type),
            self.get(pos.left()).map(|tile| tile.tile.tile_type),
            self.get(pos.right()).map(|tile| tile.tile.tile_type),
        ]
    }

    pub fn spawn_entity(
        &mut self,
        entity: Entity,
        pos: IPosition,
        position_component: &mut WriteStorage<PositionComponent>,
    ) {
        if let Some(previous_pos) = position_component
            .insert(entity, PositionComponent { pos })
            .unwrap()
        {
            panic!(
                "Cannot spawn entity at {} that already has a position {}!",
                pos, previous_pos.pos
            );
        }

        self.get_mut(pos).unwrap().entities.push(entity);
    }

    pub fn despawn_entity(
        &mut self,
        entity: Entity,
        position_component: &mut WriteStorage<PositionComponent>,
    ) {
        let pos = position_component
            .remove(entity)
            .expect("Trying to despawn entity with no position");

        let entities = &mut self.get_mut(pos.pos).unwrap().entities;

        let (index, _item) = entities
            .iter()
            .enumerate()
            .find(|(_i, item)| **item == entity)
            .unwrap();
        entities.remove(index);
    }

    pub fn buffer_idx(buffer_pos: UPosition) -> Result<usize, <usize as TryFrom<u32>>::Error> {
        Ok(usize::try_from(buffer_pos.y)? * 3 + usize::try_from(buffer_pos.x)?)
    }
}
