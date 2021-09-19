use crate::prelude::*;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct AIGoalComponent {
    pub current_goal: Option<AIGoal>,
}
