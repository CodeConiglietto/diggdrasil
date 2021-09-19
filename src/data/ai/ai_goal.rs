use specs::prelude::*;
use crate::prelude::*;

#[derive(Clone)]
pub enum AIGoal {
    // Wander,
    MoveInDirection { x: i32, y: i32 },
    PickUpItem{ item: Entity }
    // MoveToTile{x: i32, y: i32},
    // KillEntity { target: Entity },
    // AttackEntity { target: Entity },
    // AcquireFood,
    // FleeDanger,
}

impl AIGoal {
    pub fn get_textual_representation(&self, data: InputData) -> String{
        match self {
            Self::MoveInDirection {x, y} => {
                format!("Move towards {}, {}", x, y)
            }
            Self::PickUpItem{item} => {
                format!("Pick up {}", data.name.get(*item).unwrap().name)
            }
        }
    }
}