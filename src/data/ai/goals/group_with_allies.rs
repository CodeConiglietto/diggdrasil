use rand::prelude::*;
use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GroupWithAlliesGoal {
    //Child goals and data here
    pub move_in_direction_goal: Option<MoveInDirectionGoal>,
}

impl AIGoalTrait for GroupWithAlliesGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        String::from("Group with similar creatures")
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        //Find a nearby ally (Closest? Random that you can see?)
        //Determine a comfortable distance to that ally
        //If you're outside that distance, move towards that ally
        
        if let Some(this_pos) = data.position.get(parent_entity) {
            if let Some(per) = data.perception.get(parent_entity) {
                if dbg!(per.allies.len()) < 5 {//TODO: find a comfortable amount of allies
                    let closest_ally = per.allies.choose(&mut thread_rng());
                    // per.allies.iter().filter(|ally| data.position.get(**ally).is_some()).min_by_key(|a| {
                    //     let a_pos = data.position.get(**a).unwrap();
                    //     let pos_delta = a_pos.pos - this_pos.pos;
                    //     pos_delta.x.abs() + pos_delta.y.abs()
                    // });

                    if let Some(ally) = closest_ally {
                        if let Some(ally_pos) = data.position.get(*ally) {
                            if this_pos.pos.chebyshev_distance(&ally_pos.pos) > 2 {//TODO: determine a comfortable distance to ally
                                let move_in_direction = self.move_in_direction_goal.insert(MoveInDirectionGoal {
                                    direction: Direction::from_positions(ally_pos.pos, this_pos.pos),
                                    attempted: false,
                                });
                                move_in_direction.resolve(parent_entity, data)
                            } else {
                                println!("Entity is already close enough to allies");
                                Self::success()
                            }
                        } else {
                            println!("Entity attempting to group an ally that does not have a position!");
                            Self::failure()
                        }
                    } else {
                        println!("Entity attempting to group with allies has no visible allies to group with!");
                        Self::failure()
                    }
                } else {
                    println!("Entity is already close to enough allies");
                    Self::success()
                }
            } else {
                println!("Entity attempting to group with allies has no perception!");
                Self::failure()
            }
        } else {
            println!("Entity attempting to group with allies has no position!");
            Self::failure()
        }
    }
}
