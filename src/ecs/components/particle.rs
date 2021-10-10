use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct ParticleComponent {
    pub position: IPosition,
    pub height: i32,
    pub particle_type: ParticleType,
}
