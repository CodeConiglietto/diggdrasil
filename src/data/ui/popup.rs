use crate::prelude::*;

pub struct Popup {
    pub heading: String,
    pub available_goals: Vec<AIGoal>,
    pub state: PopupState,
}

impl Popup {
    pub fn render(&self, data: InputData) {
        //Heading
        //a) Goal 1
        //b) Goal 2
    }
}