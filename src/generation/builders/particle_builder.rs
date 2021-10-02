use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specs::{world::EntitiesRes, Builder, Entity, LazyUpdate};

use crate::prelude::*;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ParticleBuilder {
    Leaf {spawn_height: i32},
    Rain {wind_direction: Direction},
    Blood {spawn_height: i32},
}

impl ParticleBuilder {
    pub fn build(&self, lazy: &LazyUpdate, entities: &EntitiesRes, (x, y): (i32, i32)) -> Entity {
        match self {
            Self::Leaf {spawn_height} => 
                lazy.create_entity(&entities)
                    .with(ParticleComponent {
                        position: (x, y, *spawn_height),
                        particle_type: ParticleType::Leaf,
                    })
                    .build(),
            Self::Rain {wind_direction} => 
                lazy.create_entity(&entities)
                    .with(ParticleComponent {
                        position: (x, y,  thread_rng().gen_range(0..MAX_PARTICLE_HEIGHT)),
                        particle_type: ParticleType::Rain{
                            initial_angle: *wind_direction
                        },
                    })
                    .build(),
            Self::Blood {spawn_height} => {
                lazy.create_entity(&entities)
                    .with(ParticleComponent {
                        position: (x, y, *spawn_height),
                        particle_type: ParticleType::Blood{
                            x_vel: thread_rng().gen_range(-1..=1), 
                            y_vel: thread_rng().gen_range(-1..=1), 
                            z_vel: thread_rng().gen_range(0..=1)
                        },
                    })
                    .build()
            },
        }
    }
}
