use specs::prelude::*;

use crate::prelude::*;

pub struct MoveToEntityGoal{
    //Child goals and data here
    target: Entity,
}

impl AIGoalTrait for MoveToEntityGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: implement like the whole fuckin' thing
        //Finish
        Self::success()
    }
}