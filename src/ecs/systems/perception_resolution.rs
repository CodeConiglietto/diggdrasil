use specs::{Join, ReadStorage, WriteStorage, System};

use crate::prelude::*;

pub struct PerceptionResolutionSystem;

impl<'a> System<'a> for PerceptionResolutionSystem {
    type SystemData = (
        ReadStorage<'a, DigestionComponent>,
        ReadStorage<'a, EdibleComponent>,
        WriteStorage<'a, AIPerceptionComponent>,
        WriteStorage<'a, AIGoalComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dig, edb, mut per, mut gol) = data;

        for (dig, per, gol) in ((&dig).maybe(), &mut per, &mut gol).join() {
            per.food.clear();
            per.threats.clear();
            
            for entity in per.all.iter() {
                if edb.get(*entity).is_some() {
                    per.food.push(*entity);
                }
            }

            println!("Entity perceiving things");
            if let Some(dig) = dig {
                println!("Entity has digestion, total nutrition: {}", dig.get_total_nutrition(&edb));
                //TODO: allow for different thresholds depending on size of entity
                if dig.get_total_nutrition(&edb) < 100 {
                    println!("Entity hungry, fulfilling hunger");
                    if !gol.goal_stack.iter().any(|goal| *goal == AIGoal::FulfilHunger) {
                        gol.goal_stack.push(AIGoal::FulfilHunger);
                    }
                }
            }

            println!("Entity sees {} entities", per.all.len());
            println!("Entity sees {} food", per.food.len());
            println!("Entity sees {} threats", per.threats.len());

            per.all.clear();
        }
    }
}
