use rand::prelude::*;
use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MoveInDirectionGoal {
    //Child goals and data here
    pub direction: Direction,
}

impl AIGoalTrait for MoveInDirectionGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        format!("Move towards {:?}", self.direction)
    }

    fn resolve(&mut self, parent_entity: Entity, data: &mut GoalData) -> AIGoalResult {
        let pos = data.position.get(parent_entity).unwrap().pos;
        let offset = self.direction.get_offset();
        let new_pos = pos + offset;

        if let Some(chunk_tile) = data.tile_world.get(new_pos) {
            if !chunk_tile.tile.tile_type.collides() {
                let mut final_status = Self::failure();
                //Try attack an entity on the tile
                if !chunk_tile.entities.is_empty() {
                    for entity in chunk_tile.entities.iter() {
                        //TODO: Check if hostile
                        if data.health.get(*entity).is_some() {
                            return AttackInDirectionGoal {
                                direction: self.direction,
                            }
                            .resolve(parent_entity, data);
                        }
                    }
                }

                Self::action(AIAction::MoveInDirection { offset })
            } else {
                if offset.x.abs() == 1 && offset.y.abs() == 1 {
                    //Movement is diagonal
                    let mut axes = [Axis::X, Axis::Y];

                    if thread_rng().gen::<bool>() {
                        axes.reverse();
                    }

                    for axis in axes.iter() {
                        let mut offset = offset;

                        match axis {
                            Axis::X => {
                                offset.x = 0;
                            }
                            Axis::Y => {
                                offset.y = 0;
                            }
                        }

                        let new_pos = pos + offset;

                        if let Some(chunk_tile) = data.tile_world.get(new_pos) {
                            if !chunk_tile.tile.tile_type.collides() {
                                return Self::action(AIAction::MoveInDirection { offset });
                            }
                        }
                    }
                } else {
                    //Movement is orthogonal
                    if offset.x == 0 {
                        let mut x_vals = [1, -1];

                        if thread_rng().gen::<bool>() {
                            x_vals.reverse();
                        }

                        for val in x_vals.iter() {
                            let offset = IPosition::new(*val, offset.y);
                            let new_pos = pos + offset;

                            if let Some(chunk_tile) = data.tile_world.get(new_pos) {
                                if !chunk_tile.tile.tile_type.collides() {
                                    return Self::action(AIAction::MoveInDirection { offset });
                                }
                            }
                        }
                        //movement is about the y axis
                    } else if offset.y == 0 {
                        //movement is about the x axis
                        let mut y_vals = [1, -1];

                        if thread_rng().gen::<bool>() {
                            y_vals.reverse();
                        }

                        for val in y_vals.iter() {
                            let offset = IPosition::new(offset.x, *val);
                            let new_pos = pos + offset;

                            if let Some(chunk_tile) = data.tile_world.get(new_pos) {
                                if !chunk_tile.tile.tile_type.collides() {
                                    return Self::action(AIAction::MoveInDirection { offset });
                                }
                            }
                        }
                    }
                }
                Self::failure()
            }
        } else {
            println!("Entity attempting to move into unloaded tile!");
            Self::failure()
        }
    }
}
