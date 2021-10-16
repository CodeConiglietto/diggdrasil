use log::warn;
use ndarray::Array2;
use noise::NoiseFn;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specs::{Entity, WriteStorage};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub tiles: Array2<ChunkTile>,
}

impl Chunk {
    pub fn generate(
        &mut self,
        chunk_pos: IPosition,
        gen_package: &GenPackageResource,
        world_data: &mut WorldData,
    ) {
        let mut vegetation_local_positions = Vec::new();

        for (local_pos, chunk_tile) in self.tiles.indexed_iter_mut() {
            let local_pos = UPosition::from_idx(local_pos).unwrap();
            let pos = IPosition::global_from_local(chunk_pos, local_pos);

            let fertility = gen_package
                .fertility_noise
                .get([pos.x as f64 * 0.01, pos.y as f64 * 0.01])
                .abs();

            chunk_tile.tile = Tile {
                seed: thread_rng().gen::<usize>(),
                fertility: (fertility * 256.0) as u8,
                tile_type: if gen_package
                    .elevation_noise
                    .get([pos.x as f64 * 0.025, pos.y as f64 * 0.025])
                    > 0.25
                {
                    TileType::Wall {
                        material: Material::Stone,
                    }
                } else {
                    if fertility > thread_rng().gen_range(0.0..=2.0) {
                        vegetation_local_positions.push(local_pos);
                    }

                    TileType::Ground
                },
                tile_variant: TileVariant::default(),
            };

            if !chunk_tile.entities.is_empty() {
                warn!("Regenerating over chunk {} with entities in it!", chunk_pos);
                chunk_tile.entities.clear();
            }
        }

        for x in 12..=20 {
            for y in 12..=20 {
                self.tiles[[x, y]].tile = Tile {
                    seed: thread_rng().gen::<usize>(),
                    fertility: 0,
                    tile_type: if y != 16 && (x == 12 || x == 20 || y == 12 || y == 20) {
                        TileType::ConstructedWall {
                            material: Material::Wood,
                            material_shape: MaterialShape::Plank,
                            wall_feature: None,
                        }
                    } else {
                        TileType::Ground
                    },
                    tile_variant: TileVariant::default(),
                }
            }
        }

        let lazy = &world_data.lazy;
        let entities = &world_data.entities;

        for local_pos in vegetation_local_positions {
            if !self.tiles[local_pos.to_idx().unwrap()].tile.tile_type.collides() {
                self.spawn_entity(
                    match thread_rng().gen_range(0..=5) {
                        0 => ItemBuilder::Stick.build(lazy, entities),
                        1 => ItemBuilder::Log.build(lazy, entities),
                        2 => VegetationBuilder::Grass.build(lazy, entities),
                        // 3 => VegetationBuilder::BerryBush.build(lazy, entities),
                        // 4 => VegetationBuilder::Tree.build(lazy, entities),
                        3 => VegetationBuilder::Grass.build(lazy, entities),
                        4 => VegetationBuilder::Grass.build(lazy, entities),
                        5 => VegetationBuilder::Grass.build(lazy, entities),
                        _ => unreachable!(),
                    },
                    (chunk_pos, local_pos),
                    &mut world_data.position,
                )
            }
        }

        // for _ in 0..16 {
        //     self.spawn_somewhere_free(
        //         || ItemBuilder::Stone.build(lazy, entities),
        //         chunk_pos,
        //         &mut world_data.position,
        //     );
        // }

        for _ in 0..4 {
            self.spawn_somewhere_free(
                || CreatureBuilder::Deer.build(lazy, entities),
                chunk_pos,
                &mut world_data.position,
            );
        }
    }

    fn spawn_entity(
        &mut self,
        entity: Entity,
        (chunk_pos, local_pos): (IPosition, UPosition),
        position_component: &mut WriteStorage<PositionComponent>,
    ) {
        let pos = IPosition::global_from_local(chunk_pos, local_pos);

        assert!(
            position_component
                .insert(entity, PositionComponent { pos })
                .unwrap()
                .is_none(),
            "Cannot spawn entity that already has a position!"
        );

        self.tiles[local_pos.to_idx().unwrap()]
            .entities
            .push(entity);
    }

    fn spawn_somewhere_free<F>(
        &mut self,
        f: F,
        chunk_pos: IPosition,
        position_component: &mut WriteStorage<PositionComponent>,
    ) where
        F: FnOnce() -> Entity,
    {
        for _ in 0..10 {
            let local_pos = UPosition::new(
                thread_rng().gen_range(0..CHUNK_SIZE as u32),
                thread_rng().gen_range(0..CHUNK_SIZE as u32),
            );

            let chunk_tile = &self.tiles[local_pos.to_idx().unwrap()];

            match chunk_tile.tile.tile_type {
                TileType::Ground => {
                    if chunk_tile.entities.is_empty() {
                        self.spawn_entity(f(), (chunk_pos, local_pos), position_component);
                        return;
                    }
                }
                _ => {}
            }
        }

        warn!("Couldn't find anywhere to spawn entity!");
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            tiles: Array2::default((CHUNK_SIZE, CHUNK_SIZE)),
        }
    }
}
