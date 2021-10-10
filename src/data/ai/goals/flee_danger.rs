use specs::prelude::*;

use crate::prelude::*;

pub struct FleeDangerGoal {
    //Child goals and data here
}

impl AIGoalTrait for FleeDangerGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        if let Some(this_perc) = perc {
            if this_perc.threats.len() > 0 {
                //The average position of threats
                //TODO: make this scale depending on how far away the threats are
                let (mut ax, mut ay) = (0, 0);
                for threat in &this_perc.threats {
                    if let Some(threat_pos) = pos.get(*threat) {
                        ax += threat_pos.x;
                        ay += threat_pos.y;
                    }
                }

                AIGoalStatus::HasChildGoals{goals: vec![AIGoal::MoveInDirection{direction: Direction::from_positions((ax, ay), this_pos.get_pos_tuple())}]}
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