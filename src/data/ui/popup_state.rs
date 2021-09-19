use crate::prelude::*;

pub enum PopupState {
    Waiting,
    Returning(AIGoal),
    Canceling,
}