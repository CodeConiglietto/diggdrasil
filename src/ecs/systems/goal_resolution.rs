use crate::prelude::*;
use rand::prelude::*;
use specs::prelude::*;
use strum::IntoEnumIterator;

use crate::prelude::*;

pub struct GoalResolutionSystem;

impl<'a> System<'a> for GoalResolutionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AIGoalComponent>,
        WriteStorage<'a, AIActionComponent>,
        GoalData<'a>,
    );

    fn run(&mut self, (eids, gol, act, data): Self::SystemData) {
        for (eid, gol, act) in (&eids, &mut gol, &mut act).join() {
            //Check for latest goal in stack
            //Attempt to resolve goal
            //match result
            //-Action
            //--Set act.current_action to action
            //

            while act.current_action.is_none() && gol.goal_stack.len() > 0 {
                let mut current_goal = gol.goal_stack.last().unwrap();

                match current_goal.resolve(eid, data) {
                    Ok(success) => {
                        // println!(
                        //     "Goal {} {}",
                        //     current_goal.get_textual_representation(),
                        //     if success { "succeeded" } else { "failed" }
                        // );

                        gol.goal_stack.pop();
                    }
                    Err(action) => {
                        act.current_action = Some(action);
                    }
                }
            }

            // if let Some(current_goal) = gol.goal_stack.last_mut() {
            //     println!("Entity resolving goal: {:?}", current_goal);
            //     let goal_status = match current_goal {
            //         AIGoal::Wander => {}
            //         AIGoal::AttackInDirection { direction } => {}
            //         AIGoal::MoveInDirection { direction } => {}
            //         AIGoal::TravelPath { path } => {}
            //         AIGoal::TravelToPosition { target_pos } => {}
            //     };

            //     println!("Goal stack size: {}", gol.goal_stack.len());
            //     println!("Goal status is: {:?}", goal_status);

            //     match goal_status {
            //         AIGoalStatus::HasChildGoals { mut goals } => {
            //             gol.goal_stack.append(&mut goals);
            //         }
            //         AIGoalStatus::Finished | AIGoalStatus::Canceled => {
            //             gol.goal_stack.pop().unwrap();
            //         }
            //         AIGoalStatus::Continuing => (),
            //     }
            // } else {
            //     // println!("No goal, falling back on default behaviour");
            //     // if let Some(pers) = pers.get(eid) {
            //     //     if let Some(default_goal) = pers.get_default_goal(inp.is_some()) {
            //     //         gol.goal_stack.push(default_goal);
            //     //     }
            //     // }
            // }
        }
    }
}
