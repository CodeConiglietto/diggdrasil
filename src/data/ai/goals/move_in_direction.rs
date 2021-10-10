use specs::prelude::*;

use crate::prelude::*;

pub struct MoveInDirectionGoal {
    //Child goals and data here
}

impl AIGoalTrait for MoveInDirectionGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        let (x, y) = direction.get_offset();
        let (new_x, new_y) = (this_pos.x + x, this_pos.y + y);

        if let Some(chunk_tile) = twld.get((new_x, new_y)) {
            if !chunk_tile.tile.tile_type.collides() {
                let mut final_status = Self::failure();
                //Try attack an entity on the tile
                if !chunk_tile.entities.is_empty() {
                    for entity in chunk_tile.entities.iter() {
                        //TODO: Check if hostile
                        if hpc.get(*entity).is_some() {
                            final_status = AIGoalStatus::HasChildGoals{goals: vec![AIGoal::AttackInDirection{direction: *direction}]};
                        }
                    }
                }

                if act.current_action.is_none() {
                    act.current_action = Some(AIAction::MoveInDirection { x, y });
                    final_status = Self::success();
                }

                final_status
            } else {
                if x.abs() == 1 && y.abs() == 1 {
                    //Movement is diagonal
                    let mut axes = [Axis::X, Axis::Y];

                    if thread_rng().gen::<bool>() {
                        axes.reverse();
                    }

                    for axis in axes.iter() {
                        let (mut dx, mut dy) = (x, y);

                        match axis {
                            Axis::X => {
                                dx = 0;
                            }
                            Axis::Y => {
                                dy = 0;
                            }
                        }

                        let (new_x, new_y) = (this_pos.x + dx, this_pos.y + dy);

                        if let Some(chunk_tile) = twld.get((new_x, new_y)) {
                            if !chunk_tile.tile.tile_type.collides() {
                                Self::action(AIAction::MoveInDirection {
                                        x: dx,
                                        y: dy,
                                    });
                            }
                        }

                        if act.current_action.is_some() {
                            break;
                        }
                    }
                } else {
                    //Movement is orthogonal
                    if x == 0 {
                        let mut x_vals = [1, -1];

                        if thread_rng().gen::<bool>() {
                            x_vals.reverse();
                        }

                        for val in x_vals.iter() {
                            let (dx, dy) = (*val, y);
                            let (new_x, new_y) = (this_pos.x + val, this_pos.y + y);

                            if let Some(chunk_tile) = twld.get((new_x, new_y)) {
                                if !chunk_tile.tile.tile_type.collides() {
                                    Self::action(AIAction::MoveInDirection {
                                            x: dx,
                                            y: dy,
                                        });
                                }
                            }

                            if act.current_action.is_some() {
                                break;
                            }
                        }
                        //movement is about the y axis
                    } else if y == 0 {
                        //movement is about the x axis
                        let mut y_vals = [1, -1];

                        if thread_rng().gen::<bool>() {
                            y_vals.reverse();
                        }

                        for val in y_vals.iter() {
                            let (dx, dy) = (x, *val);
                            let (new_x, new_y) = (this_pos.x + dx, this_pos.y + dy);

                            if let Some(chunk_tile) = twld.get((new_x, new_y)) {
                                if !chunk_tile.tile.tile_type.collides() {
                                    Self::action(AIAction::MoveInDirection {
                                            x: dx,
                                            y: dy,
                                        });
                                }
                            }

                            if act.current_action.is_some() {
                                break;
                            }
                        }
                    }
                }
                Self::success()
            }
        } else {
            println!("Entity attempting to move into unloaded tile!");
            Self::failure()
        }
    }
}