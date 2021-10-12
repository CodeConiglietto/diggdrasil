use rand::prelude::*;
use specs::prelude::*;

use crate::prelude::*;

pub struct FulfilHungerGoal {
    //Child goals and data here
    pub eat_food_goal: Option<EatGoal>,
}

impl AIGoalTrait for FulfilHungerGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: add some greed or stomach size percentage to determine whether the creature is sated or not
        if let Some(dig) = data.digestion.get(parent_entity) {
            if dig.get_total_nutrition(&data.edible) >= 100 {
                Self::success()
            } else {
                let mut food = None;
                
                if let Some(inv) = data.inventory.get(parent_entity) {
                    food = inv.items.iter().filter_map(|item| *item).find(|item| data.edible.get(*item).is_some());
                }

                if food.is_none() {
                    if let Some(perc) = data.perception.get(parent_entity) {
                        food = perc.food.choose(&mut thread_rng()).copied();
                    }
                }

                if let Some(food) = food {
                    self.eat_food_goal
                        .get_or_insert_with(
                            || EatGoal{
                                target: food,
                                eat_from_inventory_goal: None,
                                eat_from_world_goal: None,
                            }
                            )
                            .resolve(parent_entity, data)
                } else {
                    //TODO: change this to be a search for food goal
                    WanderGoal{}.resolve(parent_entity, data)
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