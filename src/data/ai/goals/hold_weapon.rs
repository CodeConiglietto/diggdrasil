use std::Err as ReturnAction;

use specs::prelude::*;

use crate::prelude::*;

pub struct HoldWeaponGoal {

}

impl AIGoalTrait for HoldWeaponGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: actually implement this
        Self::success()
    }
}