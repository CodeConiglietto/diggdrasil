use specs::prelude::*;

use crate::prelude::*;

pub struct TravelToPostionGoal {
    target_pos: IPosition,
    travel_path: Option<TravelPathGoal>,
}

impl AIGoalTrait for TravelToPostionGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        let pos = data.position.get(parent_entity).unwrap().pos;

        if pos == self.target_pos {
            return Self::success();
        }

        if let Some(travel_path) = &mut self.travel_path {
            if travel_path.resolve(parent_entity, data)? {
                return Self::success();
            }
        }

        // If the previously computed path failed or we don't have one, compute a new one
        if let Some(pathing) = data.pathing.get(parent_entity) {
            if let Some(path) = pathing.pathfind(&data.tile_world, pos, self.target_pos) {
                let travel_path = self.travel_path.insert(TravelPathGoal::new(path));
                travel_path.resolve(parent_entity, data)
            } else {
                println!("Entity cannot path to {:?}", self.target_pos);
                Self::failure()
            }
        } else {
            println!("Entity tried to pathfind without knowing how to");
            Self::failure()
        }

        //If this position is adjacent:
        //-Create child goal "Move in direction"
        //If position is pathable:
        //-calculate path to position
        //-create child goal "Travel path" with calculated path
    }
}
