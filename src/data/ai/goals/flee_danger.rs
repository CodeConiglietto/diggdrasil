use specs::prelude::*;

use crate::prelude::*;

pub struct FleeDangerGoal {
    //Child goals and data here
}

impl AIGoalTrait for FleeDangerGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        let this_pos = data.position.get(parent_entity).unwrap();

        if let Some(this_perc) = data.perception.get(parent_entity) {
            if this_perc.threats.len() > 0 {
                //The average position of threats
                //TODO: make this scale depending on how far away the threats are
                let (mut ax, mut ay) = (0, 0);
                for threat in &this_perc.threats {
                    if let Some(threat_pos) = data.position.get(*threat) {
                        ax += threat_pos.x;
                        ay += threat_pos.y;
                    }
                }

                MoveInDirectionGoal{direction: Direction::from_positions((ax, ay), this_pos.get_pos_tuple())}.resolve(parent_entity, data)
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