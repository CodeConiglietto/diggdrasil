use rand::prelude::*;
use specs::{Builder, Entity, World as ECSWorld, WorldExt as ECSWorldExt};

use crate::prelude::*;

pub enum ItemBuilder {
    Stick,
    Log,
    Stone,
    Berry,
}

impl ItemBuilder {
    pub fn build(&self, ecs_world: &mut ECSWorld) -> Entity {
        let builder = match self {
            Self::Stick => ecs_world
                .create_entity()
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
                }),
            Self::Log => ecs_world
                .create_entity()
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
                }),
            Self::Stone => ecs_world
                .create_entity()
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
                }),
            Self::Berry => ecs_world
                .create_entity()
                .with(DrawComponent {
                    seed: thread_rng().gen::<usize>(),
                    sprite_builder: SpriteBuilder::Berry,
                    symbol_builder: Some(SymbolBuilder::Berry),
                })
                .with(ItemComponent)
                .with(EdibleComponent {
                    nutrient_value: 1000,
                })
                .with(NameComponent {
                    name: String::from("berry"),
                }),
        };

        builder.build()
    }
}