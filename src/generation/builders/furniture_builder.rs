use specs::{Builder, Entity, World as ECSWorld, WorldExt as ECSWorldExt};

use crate::prelude::*;

pub enum FurnitureBuilder {
    CampFire,
}

impl FurnitureBuilder {
    pub fn build(&self, ecs_world: &mut ECSWorld) -> Entity {
        let builder = match self {
            Self::CampFire => ecs_world.create_entity().with(ParticleEmitterComponent {
                particle_type: ParticleType::Smoke { lifetime: 0 },
            }),
        };

        builder.build()
    }
}
