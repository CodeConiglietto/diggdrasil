use std::{
    borrow::Cow,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use log::{debug, trace};
use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::prelude::*;

pub struct WorldMaintenanceSystem {
    pub save_buf: Vec<u8>,
    pub ids: Vec<u64>,
}

impl<'a> System<'a> for WorldMaintenanceSystem {
    type SystemData = (
        ReadExpect<'a, GenPackageResource>,
        WorldData<'a>,
        WriteExpect<'a, TileWorldResource>,
        WriteExpect<'a, IdGeneratorResource>,
        WriteExpect<'a, PendingLoadResource>,
        ReadStorage<'a, InputComponent>,
        WriteStorage<'a, ToSaveComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            gpac,
            mut world_data,
            mut twld,
            mut id_generator,
            mut pending_load,
            input,
            mut to_save,
        ) = data;

        assert!((&to_save).join().next().is_none());

        if let Some((_input, position)) = (&input, &world_data.position).join().next() {
            let center_pos = (position.x, position.y);

            let (offset_x, offset_y) = twld.offset;

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

                        let unload =
                            !(0..3).contains(&to_buffer_x) || !(0..3).contains(&to_buffer_y);
                        let relocate =
                            (0..3).contains(&from_buffer_x) && (0..3).contains(&from_buffer_y);

                        if unload {
                            debug!(
                                "Saving chunk ({},{}) from buffer index ({},{})",
                                chunk_x, chunk_y, buffer_x, buffer_y
                            );

                            let chunk = &mut twld.buffer[TileWorldResource::buffer_idx(
                                buffer_x as usize,
                                buffer_y as usize,
                            )];

                            self.ids.clear();
                            for chunk_tile in chunk.tiles.iter_mut() {
                                for entity in chunk_tile.entities.drain(..) {
                                    to_save.insert(entity, ToSaveComponent::default()).unwrap();

                                    let id = if let Some(id) = world_data.id.get(entity) {
                                        id.id
                                    } else {
                                        let id = id_generator.generate();
                                        world_data.id.insert(entity, IdComponent { id }).unwrap();
                                        id
                                    };

                                    self.ids.push(id);
                                }
                            }

                            let saved_chunk = SavedChunk {
                                chunk: Cow::Borrowed(chunk),
                                ids: Cow::Borrowed(&self.ids),
                            };

                            let filename = chunk_filename((chunk_x, chunk_y));

                            self.save_buf.clear();
                            serialize_data(&saved_chunk, &mut self.save_buf);

                            fs::write(filename, &self.save_buf).unwrap();
                        }

                        if relocate {
                            trace!(
                                "Relocating chunk ({},{}) from buffer index ({},{}) to ({}, {})",
                                new_chunk_x,
                                new_chunk_y,
                                from_buffer_x,
                                from_buffer_y,
                                buffer_x,
                                buffer_y,
                            );
                            twld.buffer.swap(
                                TileWorldResource::buffer_idx(buffer_x as usize, buffer_y as usize),
                                TileWorldResource::buffer_idx(
                                    from_buffer_x as usize,
                                    from_buffer_y as usize,
                                ),
                            );
                        } else {
                            let filename = chunk_filename((new_chunk_x, new_chunk_y));
                            if filename.exists() {
                                debug!(
                                    "Loading chunk ({},{}) at buffer index ({}, {}) from file",
                                    new_chunk_x, new_chunk_y, buffer_x, buffer_y
                                );

                                let chunk = &mut twld.buffer[TileWorldResource::buffer_idx(
                                    buffer_x as usize,
                                    buffer_y as usize,
                                )];

                                self.save_buf.clear();
                                {
                                    let mut file = File::open(&filename).unwrap();
                                    file.read_to_end(&mut self.save_buf).unwrap();
                                }
                                fs::remove_file(&filename).unwrap();

                                let saved_chunk: SavedChunk = deserialize_data(&self.save_buf);
                                *chunk = saved_chunk.chunk.into_owned();
                                pending_load.ids.extend(saved_chunk.ids.iter().copied());
                            } else {
                                debug!(
                                    "Generating chunk ({},{}) at buffer index ({},{})",
                                    new_chunk_x, new_chunk_y, buffer_x, buffer_y
                                );

                                twld.buffer[TileWorldResource::buffer_idx(
                                    buffer_x as usize,
                                    buffer_y as usize,
                                )]
                                .generate(
                                    (new_chunk_x, new_chunk_y),
                                    &gpac,
                                    &mut world_data,
                                );
                            }
                        }
                    }
                }

                twld.offset = (new_offset_x, new_offset_y);

                // TODO Optimize this to only recompute variants on new chunks and tiles adjacent to them rather than everywhere

                let (left, top) = local_to_global_position((new_offset_x, new_offset_y), (0, 0));
                let (right, bottom) =
                    local_to_global_position((new_offset_x + 3, new_offset_y + 3), (0, 0));

                for x in left..right {
                    for y in top..bottom {
                        twld.refresh_tile_variant((x, y));
                    }
                }
            }
        }
    }
}

fn chunk_filename((chunk_x, chunk_y): (i32, i32)) -> PathBuf {
    save_path().join(format!("c_{:+04}_{:+04}.bin", chunk_x, chunk_y))
}
