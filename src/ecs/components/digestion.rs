use specs::{Component, Entity, ReadStorage, VecStorage};

use crate::prelude::*;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct DigestionComponent {
    pub contents: Vec<Entity>,
}

impl DigestionComponent {
    pub fn get_total_nutrition(&self, edc: &ReadStorage<EdibleComponent>) -> usize {
        self.contents
            .iter()
            .map(|item| {
                if let Some(edible) = edc.get(*item) {
                    edible.nutrient_value
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn insert(&mut self, item: Entity) -> bool {
        self.contents.push(item);

        true
    }

    pub fn remove(&mut self, item: Entity) -> bool {
        let (index, _item) = self
            .contents
            .iter()
            .enumerate()
            .find(|(_i, ent)| **ent == item)
            .unwrap();
        self.contents.remove(index);

        true
    }
}
