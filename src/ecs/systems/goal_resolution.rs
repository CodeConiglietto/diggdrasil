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
        ReadStorage<'a, InventoryComponent>,
        WriteStorage<'a, AIGoalComponent>,
        WriteStorage<'a, AIActionComponent>,
        WriteStorage<'a, InputComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (tmap, emap, pos, hpc, inv, mut gol, mut act, mut inp) = data;

        for (pos, inv, gol, act, inp) in
            (&pos, (&inv).maybe(), &mut gol, &mut act, (&mut inp).maybe()).join()
        {
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
                    }
                    AIGoal::PickUpItem { item } => {
                        act.current_action = Some(AIAction::PickUpItem { item: *item });
                    }
                    AIGoal::DropItem { item } => {
                        act.current_action = Some(AIAction::DropItem { item: *item });
                    }
                    AIGoal::Build {
                        x,
                        y,
                        tile_type,
                        consumed_entity,
                    } => {
                        if let Some(tile_type) = tile_type {
                            if let Some(consumed_entity) = consumed_entity {
                                act.current_action = Some(AIAction::BuildAtLocation {
                                    x: *x,
                                    y: *y,
                                    tile_type: *tile_type,
                                    consumed_entity: *consumed_entity,
                                });
                            } else {
                                if let Some(inp) = inp {
                                    if let Some(inv) = inv {
                                        let item_goals = inv
                                            .items
                                            .iter()
                                            .enumerate()
                                            .filter_map(|(i, slot)| {
                                                slot.map(|item| {
                                                    (
                                                        i,
                                                        AIGoal::Build {
                                                            x: *x,
                                                            y: *y,
                                                            tile_type: Some(*tile_type),
                                                            consumed_entity: Some(item),
                                                        },
                                                    )
                                                })
                                            })
                                            .map(PopupListItem::from)
                                            .collect();

                                        inp.popup = Some(Popup::list(
                                            format!(
                                                "Build with what?",
                                            ),
                                            item_goals,
                                        ));
                                    } else {
                                        println!("Entity trying to find building material doesn't have inventory component");
                                    }
                                } else {
                                    println!("Entity trying to find building material doesn't have input component");
                                }
                            }
                        } else {
                            if let Some(inp) = inp {
                                let tile_goals = tmap.contents[[*x as usize, *y as usize]]
                                    .tile_type
                                    .available_buildings()
                                    .iter()
                                    .map(|tile_type| AIGoal::Build {
                                        x: *x,
                                        y: *y,
                                        tile_type: Some(*tile_type),
                                        consumed_entity: consumed_entity.clone(),
                                    })
                                    .enumerate()
                                    .map(PopupListItem::from)
                                    .collect();

                                inp.popup = Some(Popup::list(
                                    format!("Build what?"),
                                    tile_goals,
                                ));
                            } else {
                                println!(
                                    "Entity trying to decide building doesn't have input component"
                                );
                            }
                        }

                        //Check that x, y is adjacent to entity position
                        //Look at tile that is at x, y location
                        //Check that the tile that is there can transition to the desired tile_type

                        //Check that the consumed entity is one of:
                        //-Adjacent
                        //-Held
                        //-In inventory

                        //act.current_action{x, y, tile_type, comsumed_entities }
                    }
                }
            }
            //Assume goal is resolved for now
            gol.current_goal = None;
        }
    }
}
