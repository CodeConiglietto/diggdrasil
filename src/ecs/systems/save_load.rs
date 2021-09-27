use std::{
    convert::Infallible,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use specs::{
    saveload::{DeserializeComponents, MarkerAllocator, SerializeComponents},
    BitSet, Entities, Entity, Join, ReadStorage, System, WriteExpect, WriteStorage,
};

use crate::prelude::*;

pub struct SaveLoadSystem {
    pub bitset: BitSet,
    pub to_save: Vec<Entity>,
    pub save_buf: Vec<u8>,
}

impl<'a> System<'a> for SaveLoadSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, ToSaveComponent>,
        WriteStorage<'a, SaveMarkerComponent>,
        WriteExpect<'a, SaveMarkerAllocatorResource>,
        WriteExpect<'a, PendingLoadResource>,
        WriteExpect<'a, TileWorldResource>,
        SaveLoadData<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            to_save,
            mut save,
            mut save_allocator,
            mut pending_load,
            mut tile_world,
            mut save_load_data,
        ) = data;

        // Save marked entities
        for (entity, _to_save) in (&entities, &to_save).join() {
            let id = save_load_data.id.get(entity).unwrap().id;
            let filename = entity_filename(id);

            self.save_buf.clear();
            let mut serializer = serializer(&mut self.save_buf);

            assert_save_markers_are_clean(&save, &save_allocator);
            // START: Save markers are dirty
            {
                save_allocator.mark(entity, &mut save);
                SerializeComponents::<Infallible, SaveMarkerComponent>::serialize_recursive(
                    &save_load_data,
                    &entities,
                    &mut save,
                    &mut save_allocator,
                    &mut serializer,
                )
                .unwrap();

                fs::write(filename, &self.save_buf).unwrap();

                // Unmark and delete all the entities that have been saved
                self.bitset.clear();
                for (saved_entity, _persist) in (&entities, &save).join() {
                    self.bitset.add(saved_entity.id());
                }

                for (saved_entity, _) in (&entities, &self.bitset).join() {
                    save.remove(saved_entity);
                    entities.delete(saved_entity).unwrap();
                }

                save_allocator.clear();
            }
            // END: Save markers are dirty
            assert_save_markers_are_clean(&save, &save_allocator);
        }

        // Load entities
        for id in pending_load.ids.drain(..) {
            let filename = entity_filename(id);

            self.save_buf.clear();
            {
                let mut file = File::open(&filename).unwrap();
                file.read_to_end(&mut self.save_buf).unwrap();
            }

            fs::remove_file(&filename).unwrap();

            let mut deserializer = deserializer(&self.save_buf);

            assert_save_markers_are_clean(&save, &save_allocator);
            // START: Save markers are dirty
            {
                DeserializeComponents::<Infallible, SaveMarkerComponent>::deserialize(
                    &mut save_load_data,
                    &entities,
                    &mut save,
                    &mut save_allocator,
                    &mut deserializer,
                )
                .unwrap();

                // Remove marker from all loaded entities and place them in chunks
                self.bitset.clear();
                for (loaded_entity, _save, mut position) in
                    (&entities, &save, (&mut save_load_data.position).maybe()).join()
                {
                    self.bitset.add(loaded_entity.id());

                    if let Some(position) = &mut position {
                        tile_world
                            .get_mut((position.x, position.y))
                            .unwrap()
                            .entities
                            .push(loaded_entity);
                    }
                }

                for (loaded_entity, _) in (&entities, &self.bitset).join() {
                    save.remove(loaded_entity).unwrap();
                }

                save_allocator.clear();
            }
            // END: Save markers are dirty
            assert_save_markers_are_clean(&save, &save_allocator);
        }
    }
}

fn entity_filename(id: u64) -> PathBuf {
    save_path().join(format!("e_{:010}.bin", id))
}

// NOTE: This is the main invariant of our save system.
// Outside of the process of saving/loading a single entity file, no entities should have a SaveMarkerComponent,
// and the SaveMarkerAllocatorResource should be empty.
fn assert_save_markers_are_clean(
    save: &WriteStorage<SaveMarkerComponent>,
    save_allocator: &SaveMarkerAllocatorResource,
) {
    assert!((save).join().next().is_none());
    assert!(save_allocator.is_empty());
}
