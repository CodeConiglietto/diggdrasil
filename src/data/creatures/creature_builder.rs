use crate::prelude::*;
use specs::{Builder, Entity, World as ECSWorld, WorldExt as ECSWorldExt};

pub enum CreatureBuilder {
    Humanoid { race: Race },
    Tree,
    Log,
}

//To be refactored to either be split into multiple specialised builders or one very generic entity builder
impl CreatureBuilder {
    pub fn build(&self, ecs_world: &mut ECSWorld, under_player_control: bool) -> Entity {
        let mut builder = match self {
            Self::Humanoid { race } => ecs_world
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
                    value: 10,
                    max_value: 10,
                })
                .with(InventoryComponent::default())
                .with(DrawComponent {
                    sprite_builder: SpriteBuilder::Humanoid { race: *race },
                    symbol_builder: Some(SymbolBuilder::Humanoid { race: *race }),
                }),
            Self::Tree => {
                let contained_entities = vec![
                    CreatureBuilder::Log.build(ecs_world, false),
                    CreatureBuilder::Log.build(ecs_world, false),
                ];

                ecs_world
                    .create_entity()
                    .with(DrawComponent {
                        sprite_builder: SpriteBuilder::Tree,
                        symbol_builder: Some(SymbolBuilder::Tree),
                    })
                    .with(ColliderComponent)
                    .with(HealthComponent {
                        value: 10,
                        max_value: 10,
                    })
                    .with(DeathComponent { contained_entities })
            }
            Self::Log => ecs_world
                .create_entity()
                .with(DrawComponent {
                    sprite_builder: SpriteBuilder::Log,
                    symbol_builder: Some(SymbolBuilder::Log),
                })
                .with(ItemComponent)
                .with(NameComponent {
                    name: String::from("log"),
                }),
        };

        if under_player_control {
            builder = builder.with(InputComponent { popup: None });
        }

        builder.build()
    }
}
