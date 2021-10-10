use specs::prelude::*;

use crate::prelude::*;

pub struct StowItemGoal {
    //Child goals and data here
}

impl AIGoalTrait for StowItemGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        if let Some(man) = man {
            if let Some(held_item) = man.held_item {
                if held_item == *item {
                    //Our item is held
                    act.current_action = Some(AIAction::StowHeldItem);
                }
            }
        }
        //If we're not holding the item to stow, then try from the ground
        if act.current_action.is_none() {
            act.current_action = Some(AIAction::StowItemFromGround { item: *item });
        }
        Self::success()
    }
}