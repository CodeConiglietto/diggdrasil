use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MoveToEntityGoal {
    //Child goals and data here
    pub target: Entity,
    pub travel_to_position_goal: Option<TravelToPositionGoal>,
}

impl AIGoalTrait for MoveToEntityGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        format!("Move to {}", data.name.get(self.target).unwrap().name)
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        //TODO: change to move to closest adjacent tile to entity
        let target_pos = data.position.get(self.target).unwrap().pos;
        
        if !self
            .travel_to_position_goal
            .get_or_insert_with(|| TravelToPositionGoal {
                target_pos,
                travel_path: None,
            })
            .resolve(parent_entity, data)?
        {
            return Self::failure();
        }

        Self::success()
    }
}
