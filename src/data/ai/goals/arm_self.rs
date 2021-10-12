use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug)]
pub struct ArmSelfGoal {}

impl AIGoalTrait for ArmSelfGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: actually implement this
        Self::success()
    }
}
