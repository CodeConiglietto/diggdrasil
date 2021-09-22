use crate::prelude::*;

pub struct PopupListItem {
    pub index: usize,
    pub goal: AIGoal,
}

impl PopupListItem {
    pub fn new(index: usize, goal: AIGoal) -> Self {
        Self { index, goal }
    }
}

impl From<(usize, AIGoal)> for PopupListItem {
    fn from((index, goal): (usize, AIGoal)) -> Self {
        Self { index, goal }
    }
}
