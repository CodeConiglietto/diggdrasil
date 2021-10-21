use rand::prelude::*;
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
            if thread_rng().gen::<bool>() {
                Some(AIGoal::GroupWithAllies(GroupWithAlliesGoal {move_in_direction_goal: None}))
            } else {
                if thread_rng().gen_range(0..10) == 0 {
                // if thread_rng().gen::<bool>() {
                    Some(AIGoal::Wander(WanderGoal {travel_to_position_goal: None}))
                } else {
                    None
                }
            }
        }
    }
}
