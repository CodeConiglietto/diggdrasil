use rand::prelude::*;
use specs::{world::EntitiesRes, Builder, Entity, LazyUpdate};

use crate::prelude::*;

pub enum CreatureBuilder {
    Humanoid { race: Race },
}

//To be refactored to either be split into multiple specialised builders or one very generic entity builder
impl CreatureBuilder {
    pub fn build(&self, lazy: &LazyUpdate, entities: &EntitiesRes) -> Entity {
        match self {
            Self::Humanoid { race } => {
                //TODO: create stomach contents from something representative of the race
                let stomach_contents = vec![
                    ItemBuilder::Berry.build(lazy, entities),
                    ItemBuilder::Berry.build(lazy, entities),
                ];
                lazy.create_entity(entities)
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
                    .build()
            }
        }
    }
}
