use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TravelToPositionGoal {
    pub target_pos: IPosition,
    pub travel_path: Option<TravelPathGoal>,
}

impl AIGoalTrait for TravelToPositionGoal {
    fn get_textual_representation(&self, _data: &RenderData) -> String {
        format!("Travel to {:?}", self.target_pos)
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        let pos = data.position.get(parent_entity).unwrap().pos;

        if pos == self.target_pos {
            println!("Entity reached target");
            return Self::success();
        }

        if let Some(travel_path) = &mut self.travel_path {
            if travel_path.resolve(parent_entity, data)? {
                println!("Entity succeeded in traveling path");
                return Self::success();
            }
        }

        // If the previously computed path failed or we don't have one, compute a new one
        if let Some(pathing) = data.pathing.get_mut(parent_entity) {
            if let Some(path) = pathing.pathfind(&data.tile_world, pos, self.target_pos) {
                println!("Entity recomputing path");
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
