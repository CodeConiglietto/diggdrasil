use rand::prelude::*;
use specs::{world::EntitiesRes, Builder, Entity, LazyUpdate};

use crate::prelude::*;

pub enum ItemBuilder {
    Stick,
    Log,
    Stone,
    Berry,
}

impl ItemBuilder {
    pub fn build(&self, lazy: &LazyUpdate, entities: &EntitiesRes) -> Entity {
        match self {
            Self::Stick => lazy
                .create_entity(entities)
                .with(DrawComponent {
                    seed: thread_rng().gen::<usize>(),
                    sprite_builder: SpriteBuilder::Stick,
                    symbol_builder: Some(SymbolBuilder::Stick),
                })
                .with(ItemComponent)
                .with(MaterialComponent {
                    material: Material::Wood,
                    shape: MaterialShape::Stick,
                    amount: 5,
                })
                .with(NameComponent {
                    name: String::from("stick"),
                })
                .build(),
            Self::Log => lazy
                .create_entity(entities)
                .with(DrawComponent {
                    seed: thread_rng().gen::<usize>(),
                    sprite_builder: SpriteBuilder::Log,
                    symbol_builder: Some(SymbolBuilder::Log),
                })
                .with(ItemComponent)
                .with(MaterialComponent {
                    material: Material::Wood,
                    shape: MaterialShape::Log,
                    amount: 5,
                })
                .with(NameComponent {
                    name: String::from("log"),
                })
                .build(),
            Self::Stone => lazy
                .create_entity(entities)
                .with(DrawComponent {
                    seed: thread_rng().gen::<usize>(),
                    sprite_builder: SpriteBuilder::Stone,
                    symbol_builder: Some(SymbolBuilder::Stone),
                })
                .with(ItemComponent)
                .with(MaterialComponent {
                    material: Material::Stone,
                    shape: MaterialShape::Rock,
                    amount: 5,
                })
                .with(NameComponent {
                    name: String::from("stone"),
                })
                .build(),
            Self::Berry => lazy
                .create_entity(entities)
                .with(DrawComponent {
                    seed: thread_rng().gen::<usize>(),
                    sprite_builder: SpriteBuilder::Berry,
                    symbol_builder: Some(SymbolBuilder::Berry),
                })
                .with(ItemComponent)
                .with(EdibleComponent {
                    nutrient_value: 100,
                })
                .with(NameComponent {
                    name: String::from("berry"),
                })
                .build(),
        }
    }
}
