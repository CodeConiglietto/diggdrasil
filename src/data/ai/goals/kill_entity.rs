use specs::prelude::*;

use crate::prelude::*;

pub struct KillEntityGoal {
    //Child goals and data here
}

impl AIGoalTrait for KillEntityGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
//If the entity has a health greater than 0
                        //Create child goal "AttackEntity" with target
                        if let Some(target_hpc) = hpc.get(*target) {
                            if target_hpc.value > 0 {
                                AIGoalStatus::HasChildGoals{goals: vec![AIGoal::AttackEntity{target: *target}]}
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