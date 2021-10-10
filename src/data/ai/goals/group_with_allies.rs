use specs::prelude::*;

use crate::prelude::*;

pub struct GroupWithAlliesGoal {
    //Child goals and data here
}

impl AIGoalTrait for GroupWithAlliesGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //Find a nearby ally (Closest? Random that you can see?)
        //Determine a comfortable distance to that ally
        //If you're outside that distance, move towards that ally
        todo!()
    }
}