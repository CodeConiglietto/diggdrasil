use crate::prelude::*;
use rand::prelude::*;
use specs::prelude::*;
use strum::IntoEnumIterator;

pub struct GoalResolutionSystem;

impl<'a> System<'a> for GoalResolutionSystem {
    type SystemData = GoalData;

    fn run(&mut self, data: Self::SystemData) {
        let (
            eids,
            crd,
            twld,
            atk,
            edb,
            dig,
            pos,
            hpc,
            inv,
            man,
            nam,
            perc,
            pers,
            mut gol,
            mut act,
            mut inp,
            mut pth,
        ) = data;

        for (eid, this_pos, inv, dig, man, perc, _pers, gol, act, inp, pth) in (
            &eids,
            &pos,
            (&inv).maybe(),
            (&dig).maybe(),
            (&man).maybe(),
            (&perc).maybe(),
            (&pers).maybe(),
            &mut gol,
            &mut act,
            (&mut inp).maybe(),
            (&mut pth).maybe(),
        )
            .join()
        {
            //Check for latest goal in stack
            //Attempt to resolve goal
            //match result
            //-Action
            //--Set act.current_action to action
            //

            if let Some(current_goal) = gol.goal_stack.last_mut() {
                println!("Entity resolving goal: {:?}", current_goal);
                let goal_status = match current_goal {
                    AIGoal::Wander => {}
                    AIGoal::AttackInDirection { direction } => {}
                    AIGoal::MoveInDirection { direction } => {}
                    AIGoal::TravelPath { path } => {}
                    AIGoal::TravelToPosition { target_pos } => {}
                    //TODO: Add better error handling and move item requests to here
                    AIGoal::StowItem { item } => {}
                    AIGoal::DropItem { item } => {}
                    //TODO: allow player to hold item from ground
                    AIGoal::HoldItem { item } => {}
                    //TODO: allow player to eat items from the ground
                    AIGoal::Eat { target } => {}
                    AIGoal::Build {
                        x,
                        y,
                        tile_type,
                        consumed_entity,
                    } => {}
                    AIGoal::Craft {
                        recipe,
                        ingredients,
                    } => {}
                    AIGoal::FulfilHunger => {}
                    AIGoal::FleeDanger => {}
                    AIGoal::GroupWithAllies => {}
                    AIGoal::AttackEntity { target } => {}
                    AIGoal::KillEntity { target } => {}
                };

                println!("Goal stack size: {}", gol.goal_stack.len());
                println!("Goal status is: {:?}", goal_status);

                match goal_status {
                    AIGoalStatus::HasChildGoals { mut goals } => {
                        gol.goal_stack.append(&mut goals);
                    }
                    AIGoalStatus::Finished | AIGoalStatus::Canceled => {
                        gol.goal_stack.pop().unwrap();
                    }
                    AIGoalStatus::Continuing => (),
                }
            } else {
                // println!("No goal, falling back on default behaviour");
                // if let Some(pers) = pers.get(eid) {
                //     if let Some(default_goal) = pers.get_default_goal(inp.is_some()) {
                //         gol.goal_stack.push(default_goal);
                //     }
                // }
            }
        }
    }
}
