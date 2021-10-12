use crate::prelude::*;
use specs::prelude::*;

pub trait AIGoalTrait {
    fn get_textual_representation(&self, data: &RenderData) -> String;
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult;

    fn success() -> AIGoalResult {
        Ok(true)
    }

    fn failure() -> AIGoalResult {
        Ok(false)
    }

    fn action(action: AIAction) -> AIGoalResult {
        Err(action)
    }
}
