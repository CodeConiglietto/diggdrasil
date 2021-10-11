use specs::prelude::*;

use crate::prelude::*;

pub struct EatFromInventoryGoal {
    //Child goals and data here
    pub target: Entity,
}

impl AIGoalTrait for EatFromInventoryGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        if let Some(inv) = data.inventory.get(parent_entity) {
            if inv.contains(self.target) {
                Self::action(AIAction::EatItemFromInventory { item: self.target })
            } else {
                println!("Entity attempting to eat item that is not in its inventory!");
                Self::failure()
            }
        } else {
            println!("Entity without inventory attempting to eat item from inventory!");
            Self::failure()
        }
    }
}