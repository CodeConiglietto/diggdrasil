use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct ParticleEmitterComponent {
    pub particle_type: ParticleType,
}
