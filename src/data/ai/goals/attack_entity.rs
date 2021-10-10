use specs::prelude::*;

use crate::prelude::*;

pub struct AttackEntityGoal{
    //Child goals and data here
    target: Entity,
    move_to_entity_goal: Option<MoveToEntityGoal>,
    attack_in_direction_goal: Option<AttackInDirectionGoal>,
}

impl AIGoalTrait for AttackEntityGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //Move there
        if self.move_to_entity_goal
            .get_or_insert_with(
                || MoveToEntityGoal{ 
                    target: self.target
                })
                .resolve(parent_entity, data)?
        {
            return Self::failure()
        }

        //Attack
        let this_pos = data.pos.get(parent_entity).unwrap().get_pos_tuple();
        let target_pos = data.pos.get(self.target).unwrap().get_pos_tuple();

        if self.attack_in_direction_goal
            .get_or_insert_with(
                || AttackInDirectionGoal{ 
                    direction: Direction::from_positions(
                        this_pos, 
                        target_pos)
                    }
                )
                .resolve(parent_entity, data)? 
        {
            return Self::failure()
        }

        //Finish
        Self::success()
    }
}