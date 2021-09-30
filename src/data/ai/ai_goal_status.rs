use crate::prelude::*;

#[derive(Debug)]
pub enum AIGoalStatus {
    Continuing,
    Finished,
    Canceled,
    HasChildGoals { goals: Vec<AIGoal> },
}
