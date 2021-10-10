use specs::prelude::*;

use crate::prelude::*;

pub struct DropItemGoal{
    //Child goals and data here
    item: Entity,
    attempted: bool,
}

impl AIGoalTrait for DropItemGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        if !self.attempted {
            Self::action(AIAction::DropItemFromInventory { item: self.item })
        } else {
            Self::success()
        }
    }
}