use rand::prelude::*;
use specs::{Builder, Entity, World as ECSWorld, WorldExt as ECSWorldExt};

use crate::prelude::*;

pub enum CreatureBuilder {
    Humanoid { race: Race },
    Tree,
    Log,
    BerryBush,
    Berry,
}

//To be refactored to either be split into multiple specialised builders or one very generic entity builder
impl CreatureBuilder {
    pub fn build(&self, ecs_world: &mut ECSWorld, under_player_control: bool) -> Entity {
        let mut builder = match self {
            Self::Humanoid { race } => {
                let stomach_contents = vec![
                    CreatureBuilder::Berry.build(ecs_world, false),
                    CreatureBuilder::Berry.build(ecs_world, false),
                ];
                ecs_world
                    .create_entity()
                    .with(VelocityComponent { x: 0, y: 0 })
                    .with(IntendedMovementComponent {
                        x_delta: 0,
                        y_delta: 0,
                        controlled: true,
                    })
                    .with(ColliderComponent)
                    .with(CollisionComponent {
                        tile_collision: None,
                        entity_collisions: Vec::new(),
                    })
                    .with(AIActionComponent {
                        current_action: None,
                    })
                    .with(AIGoalComponent { current_goal: None })
                    .with(HealthComponent {
                        hit_particle: None, //TODO: Make this blood
                        turn_damage: 0,
                        value: 100,
                        max_value: 100,
                    })
                    .with(InventoryComponent::default())
                    .with(DigestionComponent {
                        contents: stomach_contents,
                    })
                    .with(DrawComponent {
                        seed: thread_rng().gen::<usize>(),
                        sprite_builder: SpriteBuilder::Humanoid { race: *race },
                        symbol_builder: Some(SymbolBuilder::Humanoid { race: *race }),
                    })
                    .with(DeathComponent {
                        contained_entities: Vec::new(),
                    })
            }
            Self::Tree => {
                let contained_entities = vec![
                    CreatureBuilder::Log.build(ecs_world, false),
                    CreatureBuilder::Log.build(ecs_world, false),
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
            Self::BerryBush => {
                let contained_entities = vec![
                    CreatureBuilder::Berry.build(ecs_world, false),
                    CreatureBuilder::Berry.build(ecs_world, false),
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

        if under_player_control {
            builder = builder.with(InputComponent { popup: None });
        }

        builder.build()
    }
}
