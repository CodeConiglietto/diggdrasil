use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DropItemGoal {
    //Child goals and data here
    pub item: Entity,
    pub attempted: bool,
}

impl AIGoalTrait for DropItemGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        format!("Drop {}", data.name.get(self.item).unwrap().name)
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        if !self.attempted {
            Self::action(AIAction::DropItemFromInventory { item: self.item })
        } else {
            Self::success()
        }
    }
}
