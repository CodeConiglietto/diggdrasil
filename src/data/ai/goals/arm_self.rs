use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ArmSelfGoal {}

impl AIGoalTrait for ArmSelfGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        String::from("Arm self")
    }

    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: actually implement this
        Self::success()
    }
}
