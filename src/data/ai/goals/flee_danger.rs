use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FleeDangerGoal {
    //Child goals and data here
}

impl AIGoalTrait for FleeDangerGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        String::from("Flee from danger")
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        let pos = data.position.get(parent_entity).unwrap().pos;

        if let Some(this_perc) = data.perception.get(parent_entity) {
            if this_perc.threats.len() > 0 {
                //The average position of threats
                //TODO: make this scale depending on how far away the threats are
                let sum_pos = this_perc
                    .threats
                    .iter()
                    .filter_map(|threat| data.position.get(*threat))
                    .map(|pos| pos.pos)
                    .sum();

                MoveInDirectionGoal {
                    direction: Direction::from_positions(pos, sum_pos),
                }
                .resolve(parent_entity, data)
            } else {
                println!("Entity has no threats to flee from!");
                Self::success()
            }
        } else {
            println!("Entity attempting to flee danger has no perception component!");
            Self::failure()
        }
    }
}
