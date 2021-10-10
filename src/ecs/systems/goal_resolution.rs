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
        ReadStorage<'a, DigestionComponent>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, HealthComponent>,
        ReadStorage<'a, InventoryComponent>,
        ReadStorage<'a, ManipulatorComponent>,
        ReadStorage<'a, NameComponent>,
        ReadStorage<'a, AIPerceptionComponent>,
        ReadStorage<'a, AIPersonalityComponent>,
        WriteStorage<'a, AIGoalComponent>,
        WriteStorage<'a, AIActionComponent>,
        WriteStorage<'a, InputComponent>,
        WriteStorage<'a, PathingComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            eids,
            crd,
            twld,
            atk,
            edb,
            dig,
            pos,
            hpc,
            inv,
            man,
            nam,
            perc,
            pers,
            mut gol,
            mut act,
            mut inp,
            mut pth,
        ) = data;

        for (eid, this_pos, inv, dig, man, perc, _pers, gol, act, inp, pth) in (
            &eids,
            &pos,
            (&inv).maybe(),
            (&dig).maybe(),
            (&man).maybe(),
            (&perc).maybe(),
            (&pers).maybe(),
            &mut gol,
            &mut act,
            (&mut inp).maybe(),
            (&mut pth).maybe(),
        )
            .join()
        {
            if let Some(current_goal) = gol.goal_stack.last_mut() {
                println!("Entity resolving goal: {:?}", current_goal);
                let goal_status = match current_goal {
                    AIGoal::Wander => {
                        act.current_action = Some(AIAction::MoveInDirection { x: thread_rng().gen_range(-1..=1), y: thread_rng().gen_range(-1..=1) });
                        AIGoalStatus::Finished
                    },
                    AIGoal::AttackInDirection { direction } => {
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

                                    AIGoalStatus::Finished
                                } else {
                                    println!("Entity attempting to attack using a weapon that has no attacks!");
                                    AIGoalStatus::Canceled
                                }
                            } else {
                                //TODO: add a basic attack to creatures, and use this here instead
                                // act.current_action =
                                //     Some(AIAction::AttackEntity {
                                //         target: *entity,
                                //     });
                                println!("Entity attempting to attack without a weapon!");
                                AIGoalStatus::Canceled
                            }
                        } else {
                            println!("Entity attempting to attack without the ability to hold a weapon!");
                            AIGoalStatus::Canceled
                        }
                    }
                    AIGoal::MoveInDirection { direction } => {
                        let (x, y) = direction.get_offset();
                        let (new_x, new_y) = (this_pos.x + x, this_pos.y + y);

                        if let Some(chunk_tile) = twld.get((new_x, new_y)) {
                            if !chunk_tile.tile.tile_type.collides() {
                                let mut final_status = AIGoalStatus::Canceled;
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
                                    final_status = AIGoalStatus::Finished;
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
                                            let (new_x, new_y) = (this_pos.x + val, this_pos.y + y);

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
                                            let (new_x, new_y) = (this_pos.x + dx, this_pos.y + dy);

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
                                AIGoalStatus::Finished
                            }
                        } else {
                            println!("Entity attempting to move into unloaded tile!");
                            AIGoalStatus::Canceled
                        }
                    }
                    AIGoal::TravelPath { path } => {
                        let position = (this_pos.x, this_pos.y);

                        //TODO: use bresenham if target position is in FOV view
                        let next_step = path.pop();

                        if let Some(next_step) = next_step {
                            if next_step == position || pos_is_adjacent(next_step, position, false) {
                                AIGoalStatus::HasChildGoals {
                                    goals: vec![AIGoal::MoveInDirection {
                                        direction: Direction::from_positions(next_step, position),
                                    }],
                                }
                            } else {
                                println!("Entity attempting to travel along path it is not adjacent to! ({:?} -> {:?})", next_step, position);
                                AIGoalStatus::Canceled
                            }
                        } else {
                            AIGoalStatus::Finished
                        }
                    }
                    AIGoal::TravelToPosition { target_pos } => {
                        let this_pos = (this_pos.x, this_pos.y);

                        if this_pos == *target_pos {
                            AIGoalStatus::Finished
                        } else {
                            if let Some(pth) = pth {
                                let path = pth.pathfind(&*twld, this_pos, *target_pos);

                                if let Some(path) = path {
                                    AIGoalStatus::HasChildGoals {
                                        goals: vec![AIGoal::TravelPath { path }],
                                    }
                                } else {
                                    println!("Entity cannot path to {:?}", target_pos);
                                    AIGoalStatus::Canceled
                                }
                            } else {
                                println!("Entity tried to pathfind without knowing how to");
                                AIGoalStatus::Canceled
                            }
                        }
                        //If this position is adjacent:
                        //-Create child goal "Move in direction"
                        //If position is pathable:
                        //-calculate path to position
                        //-create child goal "Travel path" with calculated path
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
                    AIGoal::Eat { target } => {
                        if let Some(target) = target {
                            if let Some(target_pos) = pos.get(*target) {
                                //We can assume that the item exists in the world
                                if pos_is_adjacent(this_pos.get_pos_tuple(), target_pos.get_pos_tuple(), true) {
                                    act.current_action = Some(AIAction::EatFromGround { target: *target });
                                    AIGoalStatus::Finished
                                } else {
                                    AIGoalStatus::HasChildGoals { goals: vec![AIGoal::TravelToPosition{target_pos: target_pos.get_pos_tuple()}] }
                                }
                            } else if let Some(inv) = inv {
                                if inv.contains(*target) {
                                    act.current_action =
                                        Some(AIAction::EatItemFromInventory { item: *target });

                                    AIGoalStatus::Finished
                                } else {
                                    println!("Entity attempting to eat item that is neither in the world or in its inventory!");
                                    AIGoalStatus::Canceled
                                }
                            } else {
                                println!("Entity without inventory attempting to eat item that is not in the world!");
                                AIGoalStatus::Canceled
                            }
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
                                                    (i, None, AIGoal::Eat { target: Some(item) })
                                                })
                                            })
                                        })
                                        .map(PopupListItem::from)
                                        .collect();

                                    inp.popup =
                                        Some(Popup::list(format!("Eat what?",), item_goals));
                                }
                            }
                            AIGoalStatus::Finished
                        }
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
                    AIGoal::FulfilHunger => {
                        //TODO: add some greed or stomach size percentage to determine whether the creature is sated or not
                        if let Some(dig) = dig {
                            if dig.get_total_nutrition(&edb) >= 100 {
                                AIGoalStatus::Finished
                            } else {
                                let mut food = None;
                                
                                if let Some(inv) = inv {
                                    food = inv.items.iter().filter_map(|item| *item).find(|item| edb.get(*item).is_some());
                                }

                                if food.is_none() {
                                    if let Some(perc) = perc {
                                        food = perc.food.choose(&mut thread_rng()).copied();
                                    }
                                }

                                if food.is_some() {
                                    AIGoalStatus::HasChildGoals{ goals: vec![AIGoal::Eat{ target: food }] }
                                } else {
                                    //TODO: change this to be a search for food goal
                                    AIGoalStatus::HasChildGoals{ goals: vec![AIGoal::Wander] }
                                }

                                //TODO:
                                //If food is found on entity but requires harvest:
                                //-Create child goals "Harvest" with entity and "Eat item" with item
                            }
                        } else {
                            println!("Entity is attempting to fulfil its hunger despite not having a digestion component!");
                            AIGoalStatus::Canceled
                        }
                    }
                    AIGoal::FleeDanger => {
                        if let Some(this_perc) = perc {
                            if this_perc.threats.len() > 0 {
                                //The average position of threats
                                //TODO: make this scale depending on how far away the threats are
                                let (mut ax, mut ay) = (0, 0);
                                for threat in &this_perc.threats {
                                    if let Some(threat_pos) = pos.get(*threat) {
                                        ax += threat_pos.x;
                                        ay += threat_pos.y;
                                    }
                                }

                                AIGoalStatus::HasChildGoals{goals: vec![AIGoal::MoveInDirection{direction: Direction::from_positions((ax, ay), this_pos.get_pos_tuple())}]}
                            } else {
                                println!("Entity has no threats to flee from!");
                                AIGoalStatus::Finished
                            }
                        } else {
                            println!("Entity attempting to flee danger has no perception component!");
                            AIGoalStatus::Canceled
                        }
                    }
                    AIGoal::GroupWithAllies => {
                        //Find a nearby ally (Closest? Random that you can see?)
                        //Determine a comfortable distance to that ally
                        //If you're outside that distance, move towards that ally
                        todo!()
                    }
                    AIGoal::AttackEntity {target} => {
                        let this_pos = pos.get(eid).unwrap().get_pos_tuple();
                        let target_pos = pos.get(*target).unwrap().get_pos_tuple();
                        if pos_is_adjacent(this_pos, target_pos, true) {
                            AIGoalStatus::HasChildGoals{ goals: vec![AIGoal::AttackInDirection{ direction: Direction::from_positions(this_pos, target_pos)}] }
                        } else {
                            //TODO: make this move to a position adjacent to the entity
                            AIGoalStatus::HasChildGoals{ goals: vec![AIGoal::TravelToPosition{target_pos}] }
                        }
                    }
                    AIGoal::KillEntity {target} => {
                        //If the entity has a health greater than 0
                        //Create child goal "AttackEntity" with target
                        if let Some(target_hpc) = hpc.get(*target) {
                            if target_hpc.value > 0 {
                                AIGoalStatus::HasChildGoals{goals: vec![AIGoal::AttackEntity{target: *target}]}
                            } else {
                                println!("Entity attempting to attack another entity that is already dead!");
                                AIGoalStatus::Finished
                            }
                        } else {
                            println!("Entity attempting to kill another entity that does not have any health!");
                            AIGoalStatus::Canceled
                        }
                    }
                };

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
            } else {
                // println!("No goal, falling back on default behaviour");
                // if let Some(pers) = pers.get(eid) {
                //     if let Some(default_goal) = pers.get_default_goal(inp.is_some()) {
                //         gol.goal_stack.push(default_goal);
                //     }
                // }
            }
        }
    }
}
