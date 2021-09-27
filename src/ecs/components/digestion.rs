use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use specs::{saveload::ConvertSaveload, Component, Entity, ReadStorage, VecStorage};

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

#[derive(Serialize, Deserialize)]
pub struct DigestionComponentData {
    pub contents: Vec<SaveMarkerComponent>,
}

impl ConvertSaveload<SaveMarkerComponent> for DigestionComponent {
    type Data = DigestionComponentData;
    type Error = Infallible;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<SaveMarkerComponent>,
    {
        Ok(DigestionComponentData {
            contents: self.contents.iter().map(|e| ids(*e).unwrap()).collect(),
        })
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(SaveMarkerComponent) -> Option<Entity>,
    {
        Ok(Self {
            contents: data.contents.into_iter().map(|m| ids(m).unwrap()).collect(),
        })
    }
}
