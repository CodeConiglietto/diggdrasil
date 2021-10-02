use rand::prelude::*;
use specs::{world::EntitiesRes, Builder, Entity, LazyUpdate};

use crate::prelude::*;

pub enum CreatureBuilder {
    Humanoid { race: Race },
    Deer,
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
                    .with(AIGoalComponent {
                        goal_stack: Vec::new(),
                    })
                    .with(AIPersonalityComponent {
                        diet: race.get_diet(),
                        disposition: race.get_disposition(),
                    })
                    .with(FieldOfViewComponent::new(10))
                    .with(HealthComponent {
                        hit_particle: Some(ParticleBuilder::Blood { spawn_height: 1 }),
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
                    .with(ManipulatorComponent { held_item: None })
                    .build()
            }
            Self::Deer => {
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
                    .with(AIGoalComponent {
                        goal_stack: Vec::new(),
                    })
                    .with(AIPersonalityComponent {
                        diet: Diet::Herbivorous,
                        disposition: Disposition::Timid,
                    })
                    .with(HealthComponent {
                        hit_particle: Some(ParticleBuilder::Blood { spawn_height: 1 }),
                        turn_damage: 0,
                        value: 10,
                        max_value: 10,
                    })
                    .with(DigestionComponent {
                        contents: stomach_contents,
                    })
                    .with(DrawComponent {
                        seed: thread_rng().gen::<usize>(),
                        sprite_builder: SpriteBuilder::Deer,
                        symbol_builder: Some(SymbolBuilder::Deer),
                    })
                    .with(DeathComponent {
                        //TODO: put meats
                        contained_entities: Vec::new(),
                    })
                    .build()
            }
        }
    }
}
