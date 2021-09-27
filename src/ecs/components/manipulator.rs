use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use specs::{saveload::ConvertSaveload, Component, Entity, VecStorage};

use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct ManipulatorComponent {
    pub held_item: Option<Entity>,
}

#[derive(Serialize, Deserialize)]
pub struct ManipulatorComponentData {
    pub held_item: Option<SaveMarkerComponent>,
}

impl ConvertSaveload<SaveMarkerComponent> for ManipulatorComponent {
    type Data = ManipulatorComponentData;
    type Error = Infallible;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<SaveMarkerComponent>,
    {
        Ok(ManipulatorComponentData {
            held_item: self.held_item.map(|e| ids(e).unwrap()),
        })
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(SaveMarkerComponent) -> Option<Entity>,
    {
        Ok(Self {
            held_item: data.held_item.map(|e| ids(e).unwrap()),
        })
    }
}
