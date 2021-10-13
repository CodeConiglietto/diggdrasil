use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GroupWithAlliesGoal {
    //Child goals and data here
}

impl AIGoalTrait for GroupWithAlliesGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        String::from("Group with similar creatures")
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        //Find a nearby ally (Closest? Random that you can see?)
        //Determine a comfortable distance to that ally
        //If you're outside that distance, move towards that ally
        todo!()
    }
}
