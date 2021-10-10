use std::{
    borrow::Cow,
    convert::TryFrom,
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
            let (center_chunk_pos, _) = position.pos.global_to_local();
            let new_offset = center_chunk_pos - IPosition::new(1, 1);

            if twld.offset != new_offset {
                let offset_diff = new_offset - twld.offset;

                // Relocate chunks
                for x in 0..3 {
                    let buffer_x = if offset_diff.x > 0 { x } else { 2 - x };

                    for y in 0..3 {
                        let buffer_y = if offset_diff.y > 0 { y } else { 2 - y };
                        let buffer_upos = UPosition::new(buffer_x, buffer_y);
                        let buffer_ipos = IPosition::try_from(buffer_upos).unwrap();
                        let chunk_pos = buffer_ipos + twld.offset;
                        let new_chunk_pos = buffer_ipos + new_offset;

                        let from_buffer_pos = buffer_ipos + offset_diff;
                        let to_buffer_pos = buffer_ipos - offset_diff;

                        let unload = !(0..3).contains(&to_buffer_pos.x)
                            || !(0..3).contains(&to_buffer_pos.y);
                        let relocate = (0..3).contains(&from_buffer_pos.x)
                            && (0..3).contains(&from_buffer_pos.y);

                        if unload {
                            debug!(
                                "Saving chunk {} from buffer index {}",
                                chunk_pos, buffer_upos
                            );

                            let chunk = &mut twld.buffer
                                [TileWorldResource::buffer_idx(buffer_upos).unwrap()];

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

                            let filename = chunk_filename(chunk_pos);

                            self.save_buf.clear();
                            serialize_data(&saved_chunk, &mut self.save_buf);

                            fs::write(filename, &self.save_buf).unwrap();
                        }

                        if relocate {
                            trace!(
                                "Relocating chunk {} from buffer index {} to {}",
                                new_chunk_pos,
                                from_buffer_pos,
                                buffer_upos,
                            );
                            twld.buffer.swap(
                                TileWorldResource::buffer_idx(buffer_upos).unwrap(),
                                TileWorldResource::buffer_idx(
                                    UPosition::try_from(from_buffer_pos).unwrap(),
                                )
                                .unwrap(),
                            );
                        } else {
                            let filename = chunk_filename(new_chunk_pos);
                            if filename.exists() {
                                debug!(
                                    "Loading chunk {} at buffer index {} from file",
                                    new_chunk_pos, buffer_upos
                                );

                                let chunk = &mut twld.buffer
                                    [TileWorldResource::buffer_idx(buffer_upos).unwrap()];

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
                                    "Generating chunk {} at buffer index {}",
                                    new_chunk_pos, buffer_upos
                                );

                                twld.buffer[TileWorldResource::buffer_idx(buffer_upos).unwrap()]
                                    .generate(new_chunk_pos, &gpac, &mut world_data);
                            }
                        }
                    }
                }

                twld.offset = new_offset;

                // TODO Optimize this to only recompute variants on new chunks and tiles adjacent to them rather than everywhere

                let top_left = IPosition::global_from_local(new_offset, UPosition::ZERO);
                let bottom_right = IPosition::global_from_local(
                    new_offset + IPosition::new(3, 3),
                    UPosition::ZERO,
                );

                for x in top_left.x..bottom_right.x {
                    for y in top_left.y..bottom_right.y {
                        twld.refresh_tile_variant(IPosition::new(x, y));
                    }
                }
            }
        }
    }
}

fn chunk_filename(chunk_pos: IPosition) -> PathBuf {
    save_path().join(format!("c_{:+04}_{:+04}.bin", chunk_pos.x, chunk_pos.y))
}
