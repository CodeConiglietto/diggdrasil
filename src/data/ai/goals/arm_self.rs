use specs::prelude::*;

use crate::prelude::*;

pub struct ArmSelfGoal {

}

impl AIGoalTrait for ArmSelfnGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: actually implement this
        Self::success()
    }
}