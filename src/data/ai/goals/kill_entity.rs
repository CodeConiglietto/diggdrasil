use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct KillEntityGoal {
    //Child goals and data here
    pub target: Entity,
    pub attack_entity_goal: Option<AttackEntityGoal>,
}

impl AIGoalTrait for KillEntityGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        format!("Kill {}", data.name.get(self.target).unwrap().name)
    }

    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //If the entity has a health greater than 0
        //Create child goal "AttackEntity" with target
        //TODO: Check that entity is within view, or reasonable to attack
        if let Some(target_hpc) = data.health.get(self.target) {
            if target_hpc.value > 0 {
                self.attack_entity_goal
                    .get_or_insert_with(|| AttackEntityGoal {
                        //Child goals and data here
                        target: self.target,
                        move_to_entity_goal: None,
                        attack_in_direction_goal: None,
                    })
                    .resolve(parent_entity, data)
            } else {
                println!("Entity attempting to attack another entity that is already dead!");
                Self::success()
            }
        } else {
            println!("Entity attempting to kill another entity that does not have any health!");
            Self::failure()
        }
    }
}
