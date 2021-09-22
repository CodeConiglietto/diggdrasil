use crate::prelude::*;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct HealthComponent {
    pub hit_particle: Option<ParticleType>,
    pub turn_damage: u32,
    pub value: u32,
    pub max_value: u32,
}
