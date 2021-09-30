use rand::prelude::*;
use specs::{world::EntitiesRes, Builder, Entity, LazyUpdate};

use crate::prelude::*;

pub enum VegetationBuilder {
    Grass,
    Tree,
    BerryBush,
}

//To be refactored to either be split into multiple specialised builders or one very generic entity builder
impl VegetationBuilder {
    pub fn build(&self, lazy: &LazyUpdate, entities: &EntitiesRes) -> Entity {
        match self {
            Self::Grass => {
                lazy.create_entity(entities)
                    .with(DrawComponent {
                        seed: thread_rng().gen::<usize>(),
                        sprite_builder: SpriteBuilder::Grass,
                        symbol_builder: Some(SymbolBuilder::Grass),
                    })
                    .with(NameComponent {
                        name: String::from("grass"),
                    })
                    .with(EdibleComponent {
                        nutrient_value: 100,
                    })
                    .build()
            }
            Self::Tree => {
                let contained_entities = vec![
                    ItemBuilder::Log.build(lazy, entities),
                    ItemBuilder::Log.build(lazy, entities),
                ];

                lazy.create_entity(entities)
                    .with(DrawComponent {
                        seed: thread_rng().gen::<usize>(),
                        sprite_builder: SpriteBuilder::Tree,
                        symbol_builder: Some(SymbolBuilder::Tree),
                    })
                    .with(NameComponent { 
                        name: String::from("tree"),
                    })
                    .with(ColliderComponent)
                    .with(HealthComponent {
                        hit_particle: Some(ParticleType::Leaf),
                        turn_damage: 0,
                        value: 10,
                        max_value: 10,
                    })
                    .with(DeathComponent { contained_entities })
                    .build()
            }
            Self::BerryBush => {
                let contained_entities = vec![
                    ItemBuilder::Berry.build(lazy, entities),
                    ItemBuilder::Berry.build(lazy, entities),
                ];

                lazy.create_entity(entities)
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
                    .build()
            }
        }
    }
}
