use specs::prelude::*;

use crate::prelude::*;

pub struct WanderGoal {
    //Child goals and data here
}

impl AIGoalTrait for WanderGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        act.current_action = Some(AIAction::MoveInDirection { x: thread_rng().gen_range(-1..=1), y: thread_rng().gen_range(-1..=1) });
        Self::success()
    }
    
    fn is_complete(&self, parent_entity: Entity, data: GoalData) -> bool {
        true
    }

    fn is_valid(&self, parent_entity: Entity, data: GoalData) -> bool{
        true
    }
}