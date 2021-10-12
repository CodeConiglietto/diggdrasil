use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HoldItemGoal {
    //Child goals and data here
    pub item: Entity,
}

impl AIGoalTrait for HoldItemGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        format!("Hold {}", data.name.get(self.item).unwrap().name)
    }

    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        if let Some(man) = data.manipulator.get(parent_entity) {
            if let Some(held) = man.held_item {
                //TODO: make this stow, drop, or sheath the held item
                StowItemGoal { item: held }.resolve(parent_entity, data)
            } else {
                Self::action(AIAction::HoldItemFromInventory { item: self.item })
            }
        } else {
            println!("Entity attempting to equip item does not have a manipulator component");
            Self::failure()
        }
    }
}
