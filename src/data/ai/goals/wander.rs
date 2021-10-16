use rand::prelude::*;
use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WanderGoal {
    //Child goals and data here
    pub travel_to_position_goal: Option<TravelToPositionGoal>,
}

impl AIGoalTrait for WanderGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        String::from("Wander")
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        if let Some(this_pos) = data.position.get(parent_entity) {
            if let Some(travel_goal) = &mut self.travel_to_position_goal {
                travel_goal.resolve(parent_entity, data)
            } else {
                if thread_rng().gen::<bool>() {
                    if let Some(fov) = data.field_of_view.get(parent_entity) {
                        let fov_distance = fov.shadowcast.radius as i32;

                        return self.travel_to_position_goal.get_or_insert_with( ||
                            TravelToPositionGoal{target_pos: this_pos.pos + IPosition::new(
                                thread_rng().gen_range((fov_distance * -1)..=fov_distance),
                                thread_rng().gen_range((fov_distance * -1)..=fov_distance),
                            ),
                            travel_path: None}
                        ).resolve(parent_entity, data);
                    }
                }

                Self::action(AIAction::MoveInDirection {
                    offset: IPosition::new(
                        thread_rng().gen_range(-1..=1),
                        thread_rng().gen_range(-1..=1),
                    ),
                })
            }
        } else {
            println!("Entity attempting to wander without a position!");
            Self::failure()
        }
    }
}
