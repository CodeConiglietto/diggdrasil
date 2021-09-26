use specs::{world::EntitiesRes, Builder, Entity, LazyUpdate};

use crate::prelude::*;

pub enum FurnitureBuilder {
    CampFire,
}

impl FurnitureBuilder {
    pub fn build(&self, lazy: &LazyUpdate, entities: &EntitiesRes) -> Entity {
        match self {
            Self::CampFire => lazy
                .create_entity(entities)
                .with(ParticleEmitterComponent {
                    particle_type: ParticleType::Smoke { lifetime: 0 },
                })
                .build(),
        }
    }
}
