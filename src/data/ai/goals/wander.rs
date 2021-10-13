use rand::prelude::*;
use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WanderGoal {
    //Child goals and data here
}

impl AIGoalTrait for WanderGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        String::from("Wander")
    }

    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        Self::action(AIAction::MoveInDirection {
            offset: IPosition::new(
                thread_rng().gen_range(-1..=1),
                thread_rng().gen_range(-1..=1),
            ),
        })
    }

}
