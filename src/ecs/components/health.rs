use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct HealthComponent {
    pub hit_particle: Option<ParticleType>,
    pub turn_damage: u32,
    pub value: u32,
    pub max_value: u32,
}
