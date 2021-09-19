use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct ParticleComponent {
    pub position: (i32, i32, i32), //z is height
    pub particle_type: ParticleType,
}
