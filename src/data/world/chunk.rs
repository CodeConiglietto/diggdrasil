use log::warn;
use ndarray::Array2;
use noise::NoiseFn;
use rand::prelude::*;
use specs::{Entity, WriteStorage};

use crate::prelude::*;

pub struct Chunk {
    pub tiles: Array2<ChunkTile>,
}

impl Chunk {
    pub fn generate(&mut self, (chunk_x, chunk_y): (i32, i32), gen_package: &GenPackageResource, world_data: &mut WorldData) {
        for ((local_x, local_y), chunk_tile) in self.tiles.indexed_iter_mut() {
            let (x, y) = local_to_global_position((chunk_x, chunk_y), (local_x, local_y));

            chunk_tile.tile = Tile {
                seed: thread_rng().gen::<usize>(),
                fertility: (gen_package.fertility_noise.get([x as f64 * 0.05, y as f64 * 0.05]).abs() * 256 as f64) as u8,
                tile_type: if gen_package.elevation_noise.get([x as f64 * 0.05, y as f64 * 0.05]) > 0.25 {
                    // if thread_rng().gen_range(0.0..1.0) > 0.5 {
                        TileType::Wall {
                            material: Material::Stone,
                        }
                    // } else {
                    //     TileType::ConstructedWall {
                    //         material: Material::Wood,
                    //         material_shape: MaterialShape::Plank,
                    //         wall_feature: None,
                    //     }
                    // }
                } else {
                    TileType::Ground
                },
                tile_variant: TileVariant::default(),
            };

            if !chunk_tile.entities.is_empty() {
                warn!(
                    "Regenerating over chunk ({},{}) with entities in it!",
                    chunk_x, chunk_y
                );
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

        for _ in 0..16 {
            let lazy = &world_data.lazy;
            let entities = &world_data.entities;

            self.spawn_somewhere_free(
                || VegetationBuilder::Tree.build(lazy, entities),
                (chunk_x, chunk_y),
                &mut world_data.position,
            );

            self.spawn_somewhere_free(
                || VegetationBuilder::BerryBush.build(lazy, entities),
                (chunk_x, chunk_y),
                &mut world_data.position,
            );

            self.spawn_somewhere_free(
                || ItemBuilder::Stick.build(lazy, entities),
                (chunk_x, chunk_y),
                &mut world_data.position,
            );

            self.spawn_somewhere_free(
                || ItemBuilder::Log.build(lazy, entities),
                (chunk_x, chunk_y),
                &mut world_data.position,
            );

            self.spawn_somewhere_free(
                || ItemBuilder::Stone.build(lazy, entities),
                (chunk_x, chunk_y),
                &mut world_data.position,
            );
        }
    }

    pub fn unload(&mut self, world_data: &mut WorldData) {
        for chunk_tile in self.tiles.iter_mut() {
            for entity in chunk_tile.entities.drain(..) {
                world_data.entities.delete(entity).unwrap();
            }
        }
    }

    fn spawn_entity(
        &mut self,
        entity: Entity,
        (chunk_pos, (local_x, local_y)): ((i32, i32), (usize, usize)),
        position_component: &mut WriteStorage<PositionComponent>,
    ) {
        let (x, y) = local_to_global_position(chunk_pos, (local_x, local_y));

        assert!(
            position_component
                .insert(entity, PositionComponent { x, y })
                .unwrap()
                .is_none(),
            "Cannot spawn entity that already has a position!"
        );

        self.tiles[[local_x, local_y]].entities.push(entity);
    }

    fn spawn_somewhere_free<F>(
        &mut self,
        f: F,
        chunk_pos: (i32, i32),
        position_component: &mut WriteStorage<PositionComponent>,
    ) where
        F: FnOnce() -> Entity,
    {
        for _ in 0..10 {
            let (local_x, local_y) = (
                thread_rng().gen_range(0..CHUNK_SIZE),
                thread_rng().gen_range(0..CHUNK_SIZE),
            );

            let chunk_tile = &self.tiles[[local_x, local_y]];

            match chunk_tile.tile.tile_type {
                TileType::Ground => {
                    if chunk_tile.entities.is_empty() {
                        self.spawn_entity(f(), (chunk_pos, (local_x, local_y)), position_component);
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
