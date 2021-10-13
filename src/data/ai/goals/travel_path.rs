use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TravelPathGoal {
    path: Vec<IPosition>,
    move_in_direction: Option<MoveInDirectionGoal>,
}

impl TravelPathGoal {
    pub fn new(path: Vec<IPosition>) -> Self {
        Self {
            path,
            move_in_direction: None,
        }
    }
}

impl AIGoalTrait for TravelPathGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        if let Some(dest) = self.path.first() {
            format!("Travel to {:?}", dest)
        } else {
            String::from("Travel somewhere")
        }
    }

    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        let pos = data.position.get(parent_entity).unwrap().pos;

        if let Some(move_in_direction) = &mut self.move_in_direction {
            if !move_in_direction.resolve(parent_entity, data)? {
                return Self::failure();
            }
        }

        // If the move succeeded or we don't have one ready, grab the next step

        if let Some(next_step) = self.path.pop() {
            if !next_step.is_adjacent(pos) {
                return Self::failure();
            }

            let move_in_direction = self.move_in_direction.insert(MoveInDirectionGoal {
                direction: Direction::from_positions(next_step, pos),
            });
            move_in_direction.resolve(parent_entity, data)
        } else {
            Self::success()
        }
    }
}
