use specs::prelude::*;

use crate::prelude::*;

pub struct TravelToPostionGoal {
    //Child goals and data here
}

impl AIGoalTrait for TravelToPostionGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        let this_pos = (this_pos.x, this_pos.y);

        if this_pos == *target_pos {
            Self::success()
        } else {
            if let Some(pth) = pth {
                let path = pth.pathfind(&*twld, this_pos, *target_pos);

                if let Some(path) = path {
                    AIGoalStatus::HasChildGoals {
                        goals: vec![AIGoal::TravelPath { path }],
                    }
                } else {
                    println!("Entity cannot path to {:?}", target_pos);
                    Self::failure()
                }
            } else {
                println!("Entity tried to pathfind without knowing how to");
                Self::failure()
            }
        }
        //If this position is adjacent:
        //-Create child goal "Move in direction"
        //If position is pathable:
        //-calculate path to position
        //-create child goal "Travel path" with calculated path
    }
}