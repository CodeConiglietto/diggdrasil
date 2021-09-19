use specs::{Component, VecStorage};
use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct HitParticleComponent {
    particle_type: ParticleType,
}
