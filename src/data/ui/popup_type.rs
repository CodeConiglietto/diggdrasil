use crate::prelude::*;

pub enum PopupType {
    List {
        goals: Vec<PopupListItem>,
    },
    Directions {
        directions: Directions,
        f: Option<Box<dyn FnOnce(Direction) -> AIGoal + Send + Sync + 'static>>,
    },
}
