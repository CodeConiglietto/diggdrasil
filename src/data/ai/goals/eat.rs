use specs::prelude::*;

use crate::prelude::*;

pub struct EatGoal {
    //Child goals and data here
    pub target: Entity,
    pub eat_from_inventory_goal: Option<EatItemFromInventoryGoal>,
    //TODO
    // eat_from_harvestable_goal: EatFromHarvestableGoal,
    pub eat_from_ground_goal: Option<EatFromGroundGoal>,
}

impl AIGoalTrait for EatGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        let this_pos = data.pos.get(parent_entity).unwrap();
        
        if let Some(target_pos) = data.position.get(self.target) {
            //We can assume that the item exists in the world
            if pos_is_adjacent(this_pos.get_pos_tuple(), target_pos.get_pos_tuple(), true) {
                Self::action(AIAction::EatFromGround { target: self.target })
            } else {
                AIGoalStatus::HasChildGoals { goals: vec![AIGoal::TravelToPosition{target_pos: target_pos.get_pos_tuple()}] }
            }
        } else if let Some(inv) = data.inventory.get(parent_entity) {
            if inv.contains(*target) {
                Self::action(AIAction::EatItemFromInventory { item: *target });
            } else {
                println!("Entity attempting to eat item that is neither in the world or in its inventory!");
                Self::failure()
            }
        } else {
            println!("Entity without inventory attempting to eat item that is not in the world!");
            Self::failure()
        }
    }
}