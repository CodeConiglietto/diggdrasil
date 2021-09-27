use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use specs::{saveload::ConvertSaveload, Component, Entity, VecStorage};

use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct ButcherableComponent {
    pub yielded_entities: Vec<Entity>,
}

#[derive(Serialize, Deserialize)]
pub struct ButcherableComponentData {
    pub yielded_entities: Vec<SaveMarkerComponent>,
}

impl ConvertSaveload<SaveMarkerComponent> for ButcherableComponent {
    type Data = ButcherableComponentData;
    type Error = Infallible;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<SaveMarkerComponent>,
    {
        Ok(ButcherableComponentData {
            yielded_entities: self
                .yielded_entities
                .iter()
                .map(|e| ids(*e).unwrap())
                .collect(),
        })
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(SaveMarkerComponent) -> Option<Entity>,
    {
        Ok(Self {
            yielded_entities: data
                .yielded_entities
                .into_iter()
                .map(|m| ids(m).unwrap())
                .collect(),
        })
    }
}
