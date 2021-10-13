use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MoveToEntityGoal {
    //Child goals and data here
    target: Entity,
}

impl AIGoalTrait for MoveToEntityGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        format!("Move to {}", data.name.get(self.target).unwrap().name)
    }

    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: implement like the whole fuckin' thing
        //Finish
        Self::success()
    }
}
