use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EatFromWorldGoal {
    //Child goals and data here
    pub target: Entity,
    pub move_to_entity_goal: Option<MoveToEntityGoal>,
}

impl AIGoalTrait for EatFromWorldGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        format!(
            "Eat {} at {}",
            data.name.get(self.target).unwrap().name,
            data.position.get(self.target).unwrap().pos,
        )
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        let this_pos = data.position.get(parent_entity).unwrap();

        if let Some(target_pos) = data.position.get(self.target) {
            //We can assume that the item exists in the world
            if this_pos.pos.is_adjacent_or_same(target_pos.pos) {
                println!("Entity adjacent to food, eating");
                Self::action(AIAction::EatFromGround {
                    target: self.target,
                })
            } else {
                let target = self.target;
                println!("Entity moving to food");

                if !self
                    .move_to_entity_goal
                    .get_or_insert_with(|| MoveToEntityGoal {
                        target,
                        travel_to_position_goal: None,
                    })
                    .resolve(parent_entity, data)?
                {
                    println!("Entity unable to move to food");
                    return Self::failure();
                }

                println!("Entity succeeded at moving to food, but is not adjacent to food");
                //Our position was not adjacent, but our move was successful
                //Move successful implies that the position is adjacent
                unreachable!()
            }
        } else {
            println!("Entity attempting to eat item from world that is not in the world!");
            Self::failure()
        }
    }
}
