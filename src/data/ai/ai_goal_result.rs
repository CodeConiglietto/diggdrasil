// pub enum AIGoalResult {
//     PerformAction {action: AIAction},
//     // RequestInput {/*TODO: add popup or input or something*/},
//     Succeeded,
//     Failed,
// }
use crate::prelude::*;

pub type AIGoalResult = Result<bool, AIAction>;