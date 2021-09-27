use fxhash::FxHashMap;
use specs::{
    saveload::{Marker, MarkerAllocator},
    world::EntitiesRes,
    Entity, Join, ReadStorage,
};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SaveMarkerAllocatorResource {
    next: u64,
    mapping: FxHashMap<u64, Entity>,
}

impl SaveMarkerAllocatorResource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }

    pub fn clear(&mut self) {
        self.mapping.clear()
    }
}

impl MarkerAllocator<SaveMarkerComponent> for SaveMarkerAllocatorResource {
    fn allocate(&mut self, entity: Entity, id: Option<u64>) -> SaveMarkerComponent {
        let marker = if let Some(id) = id {
            if id >= self.next {
                self.next = id + 1;
            }

            SaveMarkerComponent { id }
        } else {
            let id = self.next;
            self.next += 1;
            SaveMarkerComponent { id }
        };

        self.mapping.insert(marker.id(), entity);

        marker
    }

    fn retrieve_entity_internal(&self, id: u64) -> Option<Entity> {
        self.mapping.get(&id).cloned()
    }

    fn maintain(&mut self, entities: &EntitiesRes, storage: &ReadStorage<SaveMarkerComponent>) {
        self.mapping.clear();

        for (entity, marker) in (entities, storage).join() {
            self.mapping.insert(marker.id(), entity);
        }
    }
}
