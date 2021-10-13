use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EatGoal {
    //Child goals and data here
    pub target: Entity,
    pub eat_from_inventory_goal: Option<EatFromInventoryGoal>,
    //TODO
    // eat_from_harvestable_goal: EatFromHarvestableGoal,
    pub eat_from_world_goal: Option<EatFromWorldGoal>,
}

impl AIGoalTrait for EatGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        format!("Eat {}", data.name.get(self.target).unwrap().name)
    }

    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        if self
            .eat_from_inventory_goal
            .get_or_insert_with(|| EatFromInventoryGoal {
                target: self.target,
            })
            .resolve(parent_entity, data)?
        {
            return Self::failure();
        }

        if self
            .eat_from_world_goal
            .get_or_insert_with(|| EatFromWorldGoal {
                target: self.target,
                move_to_entity_goal: None,
            })
            .resolve(parent_entity, data)?
        {
            return Self::failure();
        }

        Self::failure()
    }
}
