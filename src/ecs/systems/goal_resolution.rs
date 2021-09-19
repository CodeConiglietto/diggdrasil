use crate::prelude::*;
use rand::prelude::*;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct GoalResolutionSystem;

impl<'a> System<'a> for GoalResolutionSystem {
    type SystemData = (
        Read<'a, TileMapResource>,
        Read<'a, EntityMapResource>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, HealthComponent>,
        WriteStorage<'a, AIGoalComponent>,
        WriteStorage<'a, AIActionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (tmap, emap, pos, hpc, mut gol, mut act) = data;

        for (pos, gol, act) in (&pos, &mut gol, &mut act).join() {
            let current_goal = &gol.current_goal;

            if let Some(goal) = current_goal {
                match goal {
                    // AIGoal::Wander => {},
                    AIGoal::MoveInDirection {
                        x,
                        y, //TODO: Change to use direction enum
                    } => {
                        let (new_x, new_y) = (pos.x + x, pos.y + y);

                        if !tmap.contents[[new_x as usize, new_y as usize]]
                            .tile_type
                            .collides()
                        {
                            let entities_at_tile = &emap.contents[[new_x as usize, new_y as usize]];

                            //Try attack an entity on the tile
                            if !entities_at_tile.is_empty() {
                                for entity in entities_at_tile {
                                    if hpc.get(*entity).is_some() {
                                        act.current_action =
                                            Some(AIAction::AttackEntity { target: *entity });
                                        break;
                                    }
                                }
                            }

                            if act.current_action.is_none() {
                                act.current_action =
                                    Some(AIAction::MoveInDirection { x: *x, y: *y });
                            }
                        } else {
                            if x.abs() == 1 && y.abs() == 1 {
                                //Movement is diagonal
                                let mut axes = [Axis::X, Axis::Y];

                                if thread_rng().gen::<bool>() {
                                    axes.reverse();
                                }

                                for axis in axes.iter() {
                                    let (mut dx, mut dy) = (*x, *y);

                                    match axis {
                                        Axis::X => {
                                            dx = 0;
                                        }
                                        Axis::Y => {
                                            dy = 0;
                                        }
                                    }

                                    let (new_x, new_y) = (pos.x + dx, pos.y + dy);
                                    if !tmap.contents[[new_x as usize, new_y as usize]]
                                        .tile_type
                                        .collides()
                                    {
                                        act.current_action =
                                            Some(AIAction::MoveInDirection { x: dx, y: dy });
                                    }

                                    if act.current_action.is_some() {
                                        break;
                                    }
                                }
                            } else {
                                //Movement is orthogonal
                                if *x == 0 {
                                    let mut x_vals = [1, -1];

                                    if thread_rng().gen::<bool>() {
                                        x_vals.reverse();
                                    }

                                    for val in x_vals.iter() {
                                        let (dx, dy) = (*val, *y);
                                        let (new_x, new_y) = (pos.x + val, pos.y + y);

                                        if !tmap.contents[[new_x as usize, new_y as usize]]
                                            .tile_type
                                            .collides()
                                        {
                                            act.current_action =
                                                Some(AIAction::MoveInDirection { x: dx, y: dy });
                                        }

                                        if act.current_action.is_some() {
                                            break;
                                        }
                                    }
                                    //movement is about the y axis
                                } else if *y == 0 {
                                    //movement is about the x axis
                                    let mut y_vals = [1, -1];

                                    if thread_rng().gen::<bool>() {
                                        y_vals.reverse();
                                    }

                                    for val in y_vals.iter() {
                                        let (dx, dy) = (*x, *val);
                                        let (new_x, new_y) = (pos.x + dx, pos.y + dy);

                                        if !tmap.contents[[new_x as usize, new_y as usize]]
                                            .tile_type
                                            .collides()
                                        {
                                            act.current_action =
                                                Some(AIAction::MoveInDirection { x: dx, y: dy });
                                        }

                                        if act.current_action.is_some() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    },
                    AIGoal::PickUpItem{item} => {
                        act.current_action = Some(AIAction::PickUpItem{item: *item});
                    },
                }
            }
            //Assume goal is resolved for now
            gol.current_goal = None;
        }
    }
}
