use specs::prelude::*;

use crate::prelude::*;

pub struct StowItemGoal {
    item: Entity,
}

impl AIGoalTrait for StowItemGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        if let Some(inventory) = data.inventory.get(parent_entity) {
            if inventory.items.iter().any(|item| *item == Some(self.item)) {
                return Self::success();
            }
        } else {
            println!("Entity tried to stow item without an inventory");
            return Self::failure();
        }

        if let Some(manipulator) = data.manipulator.get(parent_entity) {
            if let Some(held_item) = manipulator.held_item {
                if held_item == self.item {
                    //Our item is held
                    return Self::action(AIAction::StowHeldItem);
                }
            }
        }

        // If we're not holding the item to stow, then try from the ground
        Self::action(AIAction::StowItemFromGround { item: self.item })
    }
}
