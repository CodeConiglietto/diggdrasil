use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use specs::{saveload::ConvertSaveload, Component, Entity, VecStorage};

use crate::prelude::*;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct InventoryComponent {
    pub items: [Option<Entity>; 10],
}

impl InventoryComponent {
    pub fn any_slot_free(&self) -> bool {
        for i in 0..self.items.len() {
            if self.items[i].is_none() {
                return true;
            }
        }

        false
    }

    pub fn insert(&mut self, item: Entity) -> bool {
        for i in 0..self.items.len() {
            if self.items[i].is_none() {
                self.items[i] = Some(item);

                return true;
            }
        }

        false
    }

    pub fn remove(&mut self, item: Entity) -> bool {
        for i in 0..self.items.len() {
            if self.items[i] == Some(item) {
                self.items[i] = None;

                return true;
            }
        }

        false
    }

    pub fn contains(&self, item: Entity) -> bool {
        self.items.iter().any(|checked_item| *checked_item == Some(item))
    }
}

#[derive(Serialize, Deserialize)]
pub struct InventoryComponentData {
    pub items: [Option<SaveMarkerComponent>; 10],
}

impl ConvertSaveload<SaveMarkerComponent> for InventoryComponent {
    type Data = InventoryComponentData;
    type Error = Infallible;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<SaveMarkerComponent>,
    {
        Ok(InventoryComponentData {
            items: [
                self.items[0].map(|e| ids(e).unwrap()),
                self.items[1].map(|e| ids(e).unwrap()),
                self.items[2].map(|e| ids(e).unwrap()),
                self.items[3].map(|e| ids(e).unwrap()),
                self.items[4].map(|e| ids(e).unwrap()),
                self.items[5].map(|e| ids(e).unwrap()),
                self.items[6].map(|e| ids(e).unwrap()),
                self.items[7].map(|e| ids(e).unwrap()),
                self.items[8].map(|e| ids(e).unwrap()),
                self.items[9].map(|e| ids(e).unwrap()),
            ],
        })
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(SaveMarkerComponent) -> Option<Entity>,
    {
        Ok(Self {
            items: [
                data.items[0].map(|e| ids(e).unwrap()),
                data.items[1].map(|e| ids(e).unwrap()),
                data.items[2].map(|e| ids(e).unwrap()),
                data.items[3].map(|e| ids(e).unwrap()),
                data.items[4].map(|e| ids(e).unwrap()),
                data.items[5].map(|e| ids(e).unwrap()),
                data.items[6].map(|e| ids(e).unwrap()),
                data.items[7].map(|e| ids(e).unwrap()),
                data.items[8].map(|e| ids(e).unwrap()),
                data.items[9].map(|e| ids(e).unwrap()),
            ],
        })
    }
}
