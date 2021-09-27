use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct ParticleEmitterComponent {
    pub particle_type: ParticleType,
}
