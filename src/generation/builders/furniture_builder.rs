use rand::prelude::*;
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
                .with(DrawComponent {
                    seed: thread_rng().gen::<usize>(),
                    sprite_builder: SpriteBuilder::CampFire,
                    symbol_builder: Some(SymbolBuilder::CampFire),
                })
                .with(ParticleEmitterComponent {
                    particle_type: ParticleType::Smoke { color_value: 0.5, lifetime: 0 },
                })
                .build(),
        }
    }
}
