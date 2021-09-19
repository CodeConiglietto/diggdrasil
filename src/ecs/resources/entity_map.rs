use crate::prelude::*;
use ndarray::prelude::*;
use specs::{Entity, WriteStorage};

#[derive(Default)]
pub struct EntityMapResource {
    //Hold an arbitrary amount of entity sprites indexed by screen position
    pub contents: Array2<Vec<Entity>>,
}

impl EntityMapResource {
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

        self.contents[[x as usize, y as usize]].push(entity);
    }

    pub fn despawn_entity(
        &mut self,
        entity: Entity,
        position_component: &mut WriteStorage<PositionComponent>,
    ) {
        let pos = position_component.remove(entity).unwrap();

        let ent_vec = &mut self.contents[[pos.x as usize, pos.y as usize]];

        let (index, _item) = ent_vec
            .iter()
            .enumerate()
            .find(|(_i, item)| **item == entity)
            .unwrap();
        ent_vec.remove(index);
    }
}
