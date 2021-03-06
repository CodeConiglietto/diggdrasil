use rand::prelude::*;
use specs::{world::EntitiesRes, Builder, Entity, LazyUpdate};

use crate::prelude::*;

pub enum CreatureBuilder {
    Humanoid { species: Species },
    Deer,
}

//To be refactored to either be split into multiple specialised builders or one very generic entity builder
impl CreatureBuilder {
    pub fn build(&self, lazy: &LazyUpdate, entities: &EntitiesRes) -> Entity {
        match self {
            Self::Humanoid { species } => {
                //TODO: create stomach contents from something representative of the race
                let stomach_contents = vec![
                    ItemBuilder::Berry.build(lazy, entities),
                    ItemBuilder::Berry.build(lazy, entities),
                    ItemBuilder::Berry.build(lazy, entities),
                    ItemBuilder::Berry.build(lazy, entities),
                ];
                lazy.create_entity(entities)
                    .with(VelocityComponent { x: 0, y: 0 })
                    .with(IntendedMovementComponent {
                        delta: IPosition::ZERO,
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
                        diet: species.get_diet(),
                        disposition: species.get_disposition(),
                    })
                    .with(AIPerceptionComponent::default())
                    .with(SpeciesComponent {
                        species: *species,
                    })
                    .with(FieldOfViewComponent::new(12))
                    .with(PathingComponent::default())
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
                        sprite_builder: SpriteBuilder::Humanoid { species: *species },
                        symbol_builder: Some(SymbolBuilder::Humanoid { species: *species }),
                    })
                    .with(DeathComponent {
                        contained_entities: Vec::new(),
                    })
                    .with(ManipulatorComponent { held_item: None })
                    .build()
            }
            Self::Deer => {
                let stomach_contents = vec![ItemBuilder::Berry.build(lazy, entities)];
                lazy.create_entity(entities)
                    .with(VelocityComponent { x: 0, y: 0 })
                    .with(IntendedMovementComponent {
                        delta: IPosition::ZERO,
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
                    .with(AIPerceptionComponent::default())
                    .with(SpeciesComponent {
                        species: Species::Deer,
                    })
                    .with(FieldOfViewComponent::new(8))
                    .with(PathingComponent::default())
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
