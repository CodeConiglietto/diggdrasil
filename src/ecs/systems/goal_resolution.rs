use crate::prelude::*;
use rand::prelude::*;
use specs::prelude::*;
use strum::IntoEnumIterator;

pub struct GoalResolutionSystem;

impl<'a> System<'a> for GoalResolutionSystem {
    type SystemData = (
        Entities<'a>,
        CraftingData<'a>,
        ReadExpect<'a, TileWorldResource>,
        ReadStorage<'a, AttackComponent>,
        ReadStorage<'a, EdibleComponent>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, HealthComponent>,
        ReadStorage<'a, InventoryComponent>,
        ReadStorage<'a, ManipulatorComponent>,
        ReadStorage<'a, NameComponent>,
        WriteStorage<'a, AIGoalComponent>,
        WriteStorage<'a, AIActionComponent>,
        WriteStorage<'a, InputComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, crd, twld, atk, edb, pos, hpc, inv, man, nam, mut gol, mut act, mut inp) = data;

        for (_eid, pos, inv, man, gol, act, inp) in (
            &eids,
            &pos,
            (&inv).maybe(),
            (&man).maybe(),
            &mut gol,
            &mut act,
            (&mut inp).maybe(),
        )
            .join()
        {
            if let Some(current_goal) = gol.goal_stack.last_mut() {
                // if let Some(goal) = current_goal {
                let goal_status = match current_goal {
                    // AIGoal::Wander => {},
                    AIGoal::MoveInDirection { direction } => {
                        let (x, y) = direction.get_offset();
                        let (new_x, new_y) = (pos.x + x, pos.y + y);

                        if let Some(chunk_tile) = twld.get((new_x, new_y)) {
                            if !chunk_tile.tile.tile_type.collides() {
                                //Try attack an entity on the tile
                                if !chunk_tile.entities.is_empty() {
                                    for entity in chunk_tile.entities.iter() {
                                        if hpc.get(*entity).is_some() {
                                            if let Some(man) = man {
                                                if let Some(wep) = man.held_item {
                                                    if let Some(atk) = atk.get(wep) {
                                                        let attack = atk
                                                            .available_attacks
                                                            .choose(&mut thread_rng())
                                                            .unwrap();

                                                        act.current_action =
                                                            Some(AIAction::AttackInDirection {
                                                                direction: *direction,
                                                                attack: attack.clone(),
                                                                attack_offsets: None,
                                                            });
                                                        // act.current_action =
                                                        //     Some(AIAction::AttackEntity { target: *entity });
                                                        break;
                                                    }
                                                } else {
                                                    //TODO: add a basic attack to creatures, and use this here instead
                                                    act.current_action =
                                                        Some(AIAction::AttackEntity {
                                                            target: *entity,
                                                        });
                                                }
                                            }
                                        }
                                    }
                                }

                                if act.current_action.is_none() {
                                    act.current_action = Some(AIAction::MoveInDirection { x, y });
                                }
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

                                        let (new_x, new_y) = (pos.x + dx, pos.y + dy);

                                        if let Some(chunk_tile) = twld.get((new_x, new_y)) {
                                            if !chunk_tile.tile.tile_type.collides() {
                                                act.current_action =
                                                    Some(AIAction::MoveInDirection {
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
                                            let (new_x, new_y) = (pos.x + val, pos.y + y);

                                            if let Some(chunk_tile) = twld.get((new_x, new_y)) {
                                                if !chunk_tile.tile.tile_type.collides() {
                                                    act.current_action =
                                                        Some(AIAction::MoveInDirection {
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
                                            let (new_x, new_y) = (pos.x + dx, pos.y + dy);

                                            if let Some(chunk_tile) = twld.get((new_x, new_y)) {
                                                if !chunk_tile.tile.tile_type.collides() {
                                                    act.current_action =
                                                        Some(AIAction::MoveInDirection {
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
                            }
                        } else {
                            println!("Entity attempting to move into unloaded tile!");
                        }

                        AIGoalStatus::Finished
                    }
                    AIGoal::TravelPath { path } => {
                        let position = (pos.x, pos.y);

                        let next_step = path.pop();

                        if let Some(next_step) = next_step {
                            if next_step == position || pos_is_adjacent(next_step, position) {
                                AIGoalStatus::HasChildGoals{goals: vec![AIGoal::MoveInDirection{direction: Direction::from_positions(next_step, position)}]}
                            } else {
                                println!("Entity attempting to travel along path it is not adjacent to! ({:?} -> {:?})", next_step, position);
                                AIGoalStatus::Canceled
                            }
                        } else {
                            AIGoalStatus::Finished
                        }
                    }
                    //TODO: Add better error handling and move item requests to here
                    AIGoal::StowItem { item } => {
                        if let Some(man) = man {
                            if let Some(held_item) = man.held_item {
                                if held_item == *item {
                                    //Our item is held
                                    act.current_action = Some(AIAction::StowHeldItem);
                                }
                            }
                        }
                        //If we're not holding the item to stow, then try from the ground
                        if act.current_action.is_none() {
                            act.current_action = Some(AIAction::StowItemFromGround { item: *item });
                        }
                        AIGoalStatus::Finished
                    }
                    AIGoal::DropItem { item } => {
                        act.current_action = Some(AIAction::DropItemFromInventory { item: *item });
                        AIGoalStatus::Finished
                    }
                    //TODO: allow player to hold item from ground
                    AIGoal::HoldItem { item } => {
                        if let Some(man) = man {
                            if let Some(held) = man.held_item {
                                //TODO: make this stow, drop, or sheath the held item
                                AIGoalStatus::HasChildGoals {
                                    goals: vec![AIGoal::StowItem { item: held }],
                                }
                            } else {
                                if let Some(item) = item {
                                    act.current_action =
                                        Some(AIAction::HoldItemFromInventory { item: *item });
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
                                                            None,
                                                            AIGoal::HoldItem { item: Some(item) },
                                                        )
                                                    })
                                                })
                                                .map(PopupListItem::from)
                                                .collect();

                                            inp.popup = Some(Popup::list(
                                                format!("Hold what?",),
                                                item_goals,
                                            ));
                                        }
                                    }
                                }
                                AIGoalStatus::Finished
                            }
                        } else {
                            println!("Entity attempting to equip item does not have a manipulator component");
                            AIGoalStatus::Canceled
                        }
                    }
                    //TODO: allow player to eat items from the ground
                    AIGoal::EatItem { item } => {
                        if let Some(item) = item {
                            act.current_action =
                                Some(AIAction::EatItemFromInventory { item: *item });
                        } else {
                            if let Some(inp) = inp {
                                if let Some(inv) = inv {
                                    let item_goals = inv
                                        .items
                                        .iter()
                                        .enumerate()
                                        .filter_map(|(i, slot)| {
                                            slot.and_then(|item| {
                                                //Only allow the player to choose things that are actually edible
                                                edb.get(item).map(|_| {
                                                    (i, None, AIGoal::EatItem { item: Some(item) })
                                                })
                                            })
                                        })
                                        .map(PopupListItem::from)
                                        .collect();

                                    inp.popup =
                                        Some(Popup::list(format!("Eat what?",), item_goals));
                                }
                            }
                        }
                        AIGoalStatus::Finished
                    }
                    AIGoal::Build {
                        x,
                        y,
                        tile_type,
                        consumed_entity,
                    } => {
                        if let Some(chunk_tile) = twld.get((*x, *y)) {
                            if let Some(inv) = inv {
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
                                            let item_goals = inv
                                                .items
                                                .iter()
                                                .enumerate()
                                                .filter_map(|(i, slot)| {
                                                    slot.and_then(|item| {
                                                        if let Some(material) =
                                                            crd.material.get(item)
                                                        {
                                                            if fulfills_material_requirements(
                                                                material,
                                                                tile_type.get_build_requirements(),
                                                            ) {
                                                                Some(PopupListItem::from((
                                                                    i,
                                                                    if let Some(item_name) =
                                                                        nam.get(item)
                                                                    {
                                                                        Some(item_name.name.clone())
                                                                    } else {
                                                                        None
                                                                    },
                                                                    AIGoal::Build {
                                                                        x: *x,
                                                                        y: *y,
                                                                        tile_type: Some(*tile_type),
                                                                        consumed_entity: Some(item),
                                                                    },
                                                                )))
                                                            } else {
                                                                None
                                                            }
                                                        } else {
                                                            None
                                                        }
                                                    })
                                                })
                                                .collect();

                                            inp.popup = Some(Popup::list(
                                                format!("Build with what?",),
                                                item_goals,
                                            ));
                                        } else {
                                            println!("Entity trying to find building material doesn't have input component");
                                        }
                                    }
                                } else {
                                    if let Some(inp) = inp {
                                        let available_materials: Vec<_> = inv
                                            .items
                                            .iter()
                                            .filter_map(|slot| {
                                                if let Some(item) = slot {
                                                    if let Some(material) = crd.material.get(*item)
                                                    {
                                                        Some(material)
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect();

                                        let tile_goals = chunk_tile
                                            .tile
                                            .tile_type
                                            .available_buildings()
                                            .iter()
                                            .filter(|building| {
                                                let build_requirements =
                                                    building.get_build_requirements();

                                                for available_material in &available_materials {
                                                    if fulfills_material_requirements(
                                                        available_material,
                                                        build_requirements,
                                                    ) {
                                                        return true;
                                                    }
                                                }

                                                false
                                            })
                                            .enumerate()
                                            .map(|(i, tile_type)| {
                                                (
                                                    i,
                                                    Some(String::from(tile_type.get_name())),
                                                    AIGoal::Build {
                                                        x: *x,
                                                        y: *y,
                                                        tile_type: Some(*tile_type),
                                                        consumed_entity: consumed_entity.clone(),
                                                    },
                                                )
                                            })
                                            .map(PopupListItem::from)
                                            .collect();

                                        inp.popup =
                                            Some(Popup::list(format!("Build what?"), tile_goals));
                                    } else {
                                        println!(
                                        "Entity trying to decide building doesn't have input component"
                                    );
                                    }
                                }
                            } else {
                                println!("Entity trying to find building material doesn't have inventory component");
                            }
                        } else {
                            println!("Entity trying to build in an unloaded tile!");
                        }
                        AIGoalStatus::Finished
                    }
                    AIGoal::Craft {
                        recipe,
                        ingredients,
                    } => {
                        if let Some(inp) = inp {
                            if let Some(inv) = inv {
                                if let Some(recipe) = recipe {
                                    let requirements = recipe.get_ingredient_requirements();
                                    let ing_len = ingredients.len();
                                    let req_len = requirements.len();

                                    if ing_len == req_len {
                                        //Check that all ingredients fulfill their respective requirements
                                        act.current_action = Some(AIAction::Craft {
                                            recipe: *recipe,
                                            ingredients: ingredients.clone(),
                                        });
                                    } else if ing_len < req_len {
                                        //Ask for next ingredient
                                        let requirement = &requirements[ing_len];

                                        let ingredient_goals = inv
                                            .items
                                            .iter()
                                            .enumerate()
                                            .filter_map(|(i, slot)| {
                                                slot.and_then(|item| {
                                                    if requirement
                                                        .requirement
                                                        .requirement_fulfilled(item, &crd)
                                                    {
                                                        let mut appended_ingredients =
                                                            ingredients.clone();
                                                        appended_ingredients.push(item);

                                                        Some(PopupListItem::from((
                                                            i,
                                                            if let Some(item_name) = nam.get(item) {
                                                                Some(item_name.name.clone())
                                                            } else {
                                                                None
                                                            },
                                                            AIGoal::Craft {
                                                                recipe: Some(*recipe),
                                                                ingredients: appended_ingredients,
                                                            },
                                                        )))
                                                    } else {
                                                        None
                                                    }
                                                })
                                            })
                                            .collect();

                                        inp.popup = Some(Popup::list(
                                            format!("Use what for {}?", requirement.part_name),
                                            ingredient_goals,
                                        ));
                                    } else {
                                        //Something is very, very wrong
                                        println!("Entity attempting to pass too many ingredients to recipe!");
                                    }
                                } else {
                                    //TODO: allow this to take from surrounding tiles
                                    //Maybe add a utility function to return all surrounding entities
                                    let craft_goals = Recipe::iter()
                                        .filter(|recipe| {
                                            recipe.fulfillable_with_inventory_contents(inv, &crd)
                                        })
                                        .enumerate()
                                        .map(|(i, recipe)| {
                                            PopupListItem::from((
                                                i,
                                                None,
                                                AIGoal::Craft {
                                                    recipe: Some(recipe),
                                                    ingredients: Vec::new(),
                                                },
                                            ))
                                        })
                                        .collect();

                                    inp.popup =
                                        Some(Popup::list(format!("Craft what?"), craft_goals));
                                }
                            } else {
                                println!("Entity attempting to find recipe ingredients without an inventory!");
                            }
                        } else {
                            println!("Entity attempting to find recipe to craft without an input component!");
                        }
                        AIGoalStatus::Finished
                    }
                };
                // }

                println!("Goal stack size: {}", gol.goal_stack.len());
                println!("Goal status is: {:?}", goal_status);

                match goal_status {
                    AIGoalStatus::HasChildGoals { mut goals } => {
                        gol.goal_stack.append(&mut goals);
                    }
                    AIGoalStatus::Finished | AIGoalStatus::Canceled => {
                        gol.goal_stack.pop().unwrap();
                    }
                    AIGoalStatus::Continuing => (),
                }

                //Assume goal is resolved for now
                // gol.current_goal = None;
            }
        }
    }
}
