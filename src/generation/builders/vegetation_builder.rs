use rand::prelude::*;
use specs::{Builder, Entity, World as ECSWorld, WorldExt as ECSWorldExt};

use crate::prelude::*;

pub enum VegetationBuilder {
    Tree,
    BerryBush,
}

//To be refactored to either be split into multiple specialised builders or one very generic entity builder
impl VegetationBuilder {
    pub fn build(&self, ecs_world: &mut ECSWorld) -> Entity {
        let builder = match self {
            Self::Tree => {
                let contained_entities = vec![
                    ItemBuilder::Log.build(ecs_world),
                    ItemBuilder::Log.build(ecs_world),
                ];

                ecs_world
                    .create_entity()
                    .with(DrawComponent {
                        seed: thread_rng().gen::<usize>(),
                        sprite_builder: SpriteBuilder::Tree,
                        symbol_builder: Some(SymbolBuilder::Tree),
                    })
                    .with(ColliderComponent)
                    .with(HealthComponent {
                        hit_particle: Some(ParticleType::Leaf),
                        turn_damage: 0,
                        value: 10,
                        max_value: 10,
                    })
                    .with(DeathComponent { contained_entities })
            }
            Self::BerryBush => {
                let contained_entities = vec![
                    ItemBuilder::Berry.build(ecs_world),
                    ItemBuilder::Berry.build(ecs_world),
                ];

                ecs_world
                    .create_entity()
                    .with(DrawComponent {
                        seed: thread_rng().gen::<usize>(),
                        sprite_builder: SpriteBuilder::BerryBush,
                        symbol_builder: Some(SymbolBuilder::BerryBush),
                    })
                    .with(NameComponent {
                        name: String::from("berry bush"),
                    })
                    .with(ColliderComponent)
                    .with(HealthComponent {
                        hit_particle: Some(ParticleType::Leaf),
                        turn_damage: 0,
                        value: 10,
                        max_value: 10,
                    })
                    .with(DeathComponent { contained_entities })
            }
        };

        builder.build()
    }
}
