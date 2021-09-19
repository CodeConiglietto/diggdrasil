use specs::Entity;

use crate::prelude::*;

#[derive(Default)]
pub struct ParticleMapResource {
    pub contents: [Vec<Entity>; MAP_Y_SIZE],
}

impl ParticleMapResource {
    pub fn clear_all(&mut self) {
        for vec in self.contents.iter_mut() {
            vec.clear();
        }
    }
}
