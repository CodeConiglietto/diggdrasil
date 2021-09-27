use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct AIGoalComponent {
    #[serde(skip)]
    pub current_goal: Option<AIGoal>,
}
