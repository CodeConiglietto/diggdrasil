use specs::prelude::*;

use crate::prelude::*;

pub struct FulfilGungerGoal {
    //Child goals and data here
}

impl AIGoalTrait for FulfilHungerGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: add some greed or stomach size percentage to determine whether the creature is sated or not
        if let Some(dig) = dig {
            if dig.get_total_nutrition(&edb) >= 100 {
                Self::success()
            } else {
                let mut food = None;
                
                if let Some(inv) = inv {
                    food = inv.items.iter().filter_map(|item| *item).find(|item| edb.get(*item).is_some());
                }

                if food.is_none() {
                    if let Some(perc) = perc {
                        food = perc.food.choose(&mut thread_rng()).copied();
                    }
                }

                if food.is_some() {
                    AIGoalStatus::HasChildGoals{ goals: vec![AIGoal::Eat{ target: food }] }
                } else {
                    //TODO: change this to be a search for food goal
                    AIGoalStatus::HasChildGoals{ goals: vec![AIGoal::Wander] }
                }

                //TODO:
                //If food is found on entity but requires harvest:
                //-Create child goals "Harvest" with entity and "Eat item" with item
            }
        } else {
            println!("Entity is attempting to fulfil its hunger despite not having a digestion component!");
            Self::failure()
        }
    }
}