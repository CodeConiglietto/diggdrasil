use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

impl RotationDirection {
    pub fn get_random() -> RotationDirection {
        if thread_rng().gen() {
            RotationDirection::Clockwise
        } else {
            RotationDirection::CounterClockwise
        }
    }
}
