use crate::prelude::*;

pub struct PopupListItem {
    pub index: usize,
    pub display_string: Option<String>,
    pub goal: AIGoal,
}

impl PopupListItem {
    pub fn new(index: usize, display_string: Option<String>, goal: AIGoal) -> Self {
        Self {
            index,
            display_string,
            goal,
        }
    }
}

impl From<(usize, Option<String>, AIGoal)> for PopupListItem {
    fn from((index, display_string, goal): (usize, Option<String>, AIGoal)) -> Self {
        Self {
            index,
            display_string,
            goal,
        }
    }
}
