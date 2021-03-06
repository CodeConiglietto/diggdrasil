use specs::{Join, Entities, ReadStorage, System, WriteStorage};

use crate::prelude::*;

pub struct PerceptionResolutionSystem;

impl<'a> System<'a> for PerceptionResolutionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, DigestionComponent>,
        ReadStorage<'a, EdibleComponent>,
        ReadStorage<'a, SpeciesComponent>,
        WriteStorage<'a, AIPerceptionComponent>,
        WriteStorage<'a, AIGoalComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, dig, edb, spc, mut per, mut gol) = data;

        for (eid, dig, per, gol) in (&eids, (&dig).maybe(), &mut per, &mut gol).join() {
            per.food.clear();
            per.allies.clear();
            per.threats.clear();

            let this_species = spc.get(eid);

            for entity in per.all.iter() {
                if edb.get(*entity).is_some() {
                    per.food.push(*entity);
                }

                //TODO: Change to a more complex hostility check
                if let Some(this_species) = this_species {
                    if let Some(other_species) = spc.get(*entity) {
                        if this_species.species == other_species.species {
                            per.allies.push(*entity);
                        }
                    }
                }
            }

            println!("Entity perceiving things");
            if let Some(dig) = dig {
                println!(
                    "Entity has digestion, total nutrition: {}",
                    dig.get_total_nutrition(&edb)
                );
                //TODO: allow for different thresholds depending on size of entity
                if dig.get_total_nutrition(&edb) < 100 {
                    println!("Entity hungry, fulfilling hunger");
                    if !gol.goal_stack.iter().any(|goal| match goal {
                        AIGoal::FulfilHunger(_) => true,
                        _ => false,
                    }) {
                        gol.goal_stack.push(AIGoal::FulfilHunger(FulfilHungerGoal {
                            eat_food_goal: None,
                        }));
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
