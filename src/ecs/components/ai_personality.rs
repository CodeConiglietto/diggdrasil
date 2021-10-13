use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct AIPersonalityComponent {
    pub diet: Diet,
    pub disposition: Disposition,
}

impl AIPersonalityComponent {
    pub fn get_default_goal(&self, under_player_control: bool) -> Option<AIGoal> {
        if under_player_control {
            None
        } else {
            Some(AIGoal::Wander(WanderGoal {}))
        }
    }
}
