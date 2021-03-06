use rand::prelude::*;
use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FulfilHungerGoal {
    //Child goals and data here
    pub eat_food_goal: Option<EatGoal>,
}

impl AIGoalTrait for FulfilHungerGoal {
    fn get_textual_representation(&self, _data: &RenderData) -> String {
        String::from("Fulfil hunger")
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        //TODO: add some greed or stomach size percentage to determine whether the creature is sated or not
        if let Some(dig) = data.digestion.get(parent_entity) {
            if dig.get_total_nutrition(&data.edible) >= 100 {
                Self::success()
            } else {
                let mut food = None;

                if let Some(inv) = data.inventory.get(parent_entity) {
                    food = inv
                        .items
                        .iter()
                        .filter_map(|item| *item)
                        .find(|item| data.edible.get(*item).is_some());
                }

                let parent_pos = data.position.get(parent_entity).unwrap();

                if food.is_none() {
                    if let Some(perc) = data.perception.get(parent_entity) {
                        // food = perc.food.choose(&mut thread_rng()).copied();
                        food = perc.food.iter().min_by_key(|a| {
                            let a_pos = data.position.get(**a).unwrap();

                            let pos_delta = a_pos.pos - parent_pos.pos;

                            pos_delta.x.abs() + pos_delta.y.abs()
                        }).copied();
                    }
                }

                if let Some(food) = food { 
                    self.eat_food_goal
                        .get_or_insert_with(|| EatGoal {
                            target: food,
                            eat_from_inventory_goal: None,
                            eat_from_world_goal: None,
                        })
                        .resolve(parent_entity, data)
                } else {
                    //TODO: change this to be a search for food goal
                    WanderGoal {travel_to_position_goal: None}.resolve(parent_entity, data)
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
