use specs::prelude::*;

use crate::prelude::*;

pub struct TravelPathGoal {
    //Child goals and data here
}

impl AIGoalTrait for TravelPathGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        let position = (this_pos.x, this_pos.y);

        //TODO: use bresenham if target position is in FOV view
        let next_step = path.pop();

        if let Some(next_step) = next_step {
            if next_step == position || pos_is_adjacent(next_step, position, false) {
                AIGoalStatus::HasChildGoals {
                    goals: vec![AIGoal::MoveInDirection {
                        direction: Direction::from_positions(next_step, position),
                    }],
                }
            } else {
                println!("Entity attempting to travel along path it is not adjacent to! ({:?} -> {:?})", next_step, position);
                Self::failure()
            }
        } else {
            Self::success()
        }
    }
}