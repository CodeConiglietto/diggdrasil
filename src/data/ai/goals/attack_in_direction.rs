use rand::prelude::*;
use specs::prelude::*;

use crate::prelude::*;

pub struct AttackInDirectionGoal{
    //Child goals and data here
    direction: Direction,
}

impl AIGoalTrait for AttackInDirectionGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: rewrite:
        //if Hold a weapon succeeds
        //-Attack using that weapon
        //else if I have an attack
        //-Attack using natural attack
        //else
        //-fail
        
        if let Some(man) = data.manipulator.get(parent_entity) {
            if let Some(wep) = man.held_item {
                if let Some(atk) = data.attack.get(wep) {
                    let attack = atk
                        .available_attacks
                        .choose(&mut thread_rng())
                        .unwrap();

                    Self::action(AIAction::AttackInDirection {
                            direction: self.direction,
                            attack: attack.clone(),
                            attack_offsets: None,
                        })
                } else {
                    println!("Entity attempting to attack using a weapon that has no attacks!");
                    Self::failure()
                }
            } else {
                //TODO: add a basic attack to creatures, and use this here instead
                // act.current_action =
                //     Some(AIAction::AttackEntity {
                //         target: *entity,
                //     });
                println!("Entity attempting to attack without a weapon!");
                Self::failure()
            }
        } else {
            println!("Entity attempting to attack without the ability to hold a weapon!");
            Self::failure()
        }
    }
}