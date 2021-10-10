use rand::prelude::*;
use specs::{Builder, Entities, Join, LazyUpdate, Read, System, WriteExpect, WriteStorage};

use crate::prelude::*;

pub struct ActionResolutionSystem;

impl<'a> System<'a> for ActionResolutionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        CraftingData<'a>,
        WriteExpect<'a, TileWorldResource>,
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, AIActionComponent>,
        WriteStorage<'a, IntendedMovementComponent>,
        WriteStorage<'a, InventoryComponent>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, DigestionComponent>,
        WriteStorage<'a, ManipulatorComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            eids,
            lup,
            crd,
            mut twld,
            mut pos,
            mut act,
            mut imc,
            mut inv,
            mut hpc,
            mut dig,
            mut man,
        ) = data;

        for (eid, act, imc, man) in (&eids, &mut act, &mut imc, (&mut man).maybe()).join() {
            let current_action = &mut act.current_action;

            //TODO: check for interruptions and then cancel action if it's interrupted
            if let Some(action) = current_action.take() {
                println!("Current action is: {:?}", action);

                act.current_action = match action {
                    AIAction::MoveInDirection { x, y } => {
                        imc.x_delta = x;
                        imc.y_delta = y;
                        imc.controlled = true;
                        None
                    }
                    AIAction::AttackInDirection {
                        direction,
                        attack,
                        attack_offsets,
                    } => {
                        let mut final_action = None;

                        let this_pos = pos.get(eid).unwrap();

                        let mut attack_offsets = attack_offsets
                            .unwrap_or_else(|| attack.attack_type.get_offsets(&direction, None));

                        if attack_offsets.len() > 0 {
                            let (offset_x, offset_y) = attack_offsets.pop().unwrap();
                            let attack_pos = (offset_x + this_pos.x, offset_y + this_pos.y);
                            let (attack_x, attack_y) = attack_pos;
                            let attack_tile = twld.get(attack_pos);
                            if let Some(attack_tile) = attack_tile {
                                for entity in &attack_tile.entities {
                                    if let Some(target_hp) = &mut hpc.get_mut(*entity) {
                                        target_hp.turn_damage += attack.attack_dice.roll();
                                        break;
                                    }
                                }

                                lup.create_entity(&eids)
                                    .with(ParticleComponent {
                                        position: (attack_x, attack_y, 0),
                                        particle_type: ParticleType::Thrust {
                                            drawn: false,
                                            direction_from_player: Direction::from_positions(
                                                (offset_x, offset_y),
                                                (0, 0),
                                            ),
                                        },
                                    })
                                    .build();

                                final_action = Some(AIAction::AttackInDirection {
                                    direction,
                                    attack,
                                    attack_offsets: Some(attack_offsets),
                                });
                            }
                        }

                        final_action
                    }
                    AIAction::AttackEntity { target } => {
                        //Will crash if attempting to attack a target without a position
                        let target_pos = pos.get(target).unwrap();
                        let this_pos = pos.get(eid).unwrap();

                        //Will not allow entity to attack a target on the same tile
                        if pos_is_adjacent(
                            (this_pos.x, this_pos.y),
                            (target_pos.x, target_pos.y),
                            false,
                        ) {
                            //Will crash if attempting to attack a target that has no health component
                            if let Some(target_hp) = &mut hpc.get_mut(target) {
                                if target_hp.value > 0 {
                                    target_hp.turn_damage += 1;
                                }
                            } else {
                                println!(
                                    "Entity attempted to attack target that has no HP component!"
                                );
                            }
                        } else {
                            println!("Entity attempted to attack target that it cannot reach!");
                        }
                        None
                    }
                    AIAction::StowItemFromGround { item } => {
                        if let Some(inventory) = inv.get_mut(eid) {
                            if let Some(entity_position) = pos.get(eid) {
                                if let Some(item_position) = pos.get(item) {
                                    if entity_position.x == item_position.x
                                        && entity_position.y == item_position.y
                                    {
                                        if inventory.insert(item) {
                                            twld.despawn_entity(item, &mut pos);
                                        } else {
                                            println!("Entity failed to stow item in inventory!");
                                        }
                                    } else {
                                        println!(
                                            "Entity attempted to stow item that it cannot reach!"
                                        );
                                    }
                                } else {
                                    println!(
                                        "Entity attempting to stow item that has no position!"
                                    );
                                }
                            } else {
                                println!(
                                    "Entity attempting to stow item despite having no position!"
                                );
                            }
                        } else {
                            println!("No inventory to store item in!");
                        }
                        None
                    }
                    AIAction::StowHeldItem => {
                        if let Some(inventory) = inv.get_mut(eid) {
                            if let Some(man) = man {
                                if let Some(held_item) = man.held_item {
                                    if inventory.insert(held_item) {
                                        man.held_item = None;
                                    } else {
                                        println!("Entity failed to stow item in inventory!");
                                    }
                                } else {
                                    println!(
                                        "Entity attempting to stow held item that is not held by that entity!"
                                    );
                                }
                            } else {
                                println!("Entity attempting to stow held item despite being unable to hold items!");
                            }
                        } else {
                            println!("No inventory to store item in!");
                        }
                        None
                    }
                    AIAction::DropItemFromInventory { item } => {
                        if let Some(inventory) = inv.get_mut(eid) {
                            if let Some(entity_position) = pos.get(eid) {
                                inventory.remove(item);
                                twld.spawn_entity(
                                    item,
                                    (entity_position.x, entity_position.y),
                                    &mut pos,
                                );
                            } else {
                                println!(
                                    "Entity attempting to drop an item despite having no position!"
                                );
                            }
                        } else {
                            println!(
                                "Entity attempting to drop an item despite having no inventory!"
                            );
                        }
                        None
                    }
                    AIAction::HoldItemFromInventory { item } => {
                        if let Some(man) = man {
                            if man.held_item.is_none() {
                                if let Some(inv) = inv.get_mut(eid) {
                                    if inv.remove(item) {
                                        man.held_item = Some(item);
                                    } else {
                                        println!("Entity attempting to hold item from inventory that is not in its inventory!");
                                    }
                                }
                            } else {
                                println!(
                                    "Entity attempting to hold item despite already holding one!"
                                );
                            }
                        } else {
                            println!(
                                "Entity attempting to hold item despite having no manipulator!"
                            );
                        }
                        None
                    }
                    AIAction::EatItemFromInventory { item } => {
                        if let Some(dig) = dig.get_mut(eid) {
                            if let Some(inv) = inv.get_mut(eid) {
                                inv.remove(item);
                                dig.insert(item);
                            }
                        }
                        None
                    }
                    AIAction::EatFromGround { target } => {
                        if let Some(dig) = dig.get_mut(eid) {
                            if let Some(this_pos) = pos.get(eid) {
                                if let Some(entity_pos) = pos.get(target) {
                                    if pos_is_adjacent(
                                        this_pos.get_pos_tuple(),
                                        entity_pos.get_pos_tuple(),
                                        true,
                                    ) {
                                        twld.despawn_entity(target, &mut pos);
                                        dig.insert(target);
                                    } else {
                                        println!("Entity attempting to eat entity from ground that it cannot reach!");
                                    }
                                } else {
                                    println!("Entity attempting to eat entity from ground that has no position component!");
                                }
                            } else {
                                println!("Entity attempting to eat from ground despite having no position component!");
                            }
                        } else {
                            println!("Entity attempting to eat from ground despite having no digestion component!");
                        }
                        None
                    }
                    AIAction::BuildAtLocation {
                        x,
                        y,
                        tile_type,
                        consumed_entity,
                    } => {
                        if let Some(chunk_tile) = twld.get((x, y)) {
                            if let Some(pos) = pos.get(eid) {
                                if pos_is_adjacent((x, y), (pos.x, pos.y), false) {
                                    if chunk_tile
                                        .tile
                                        .tile_type
                                        .available_buildings()
                                        .contains(&tile_type)
                                    {
                                        if let Some(inv) = inv.get_mut(eid) {
                                            if let Some((item_index, item)) =
                                                inv.items.iter().enumerate().find(|(_, slot)| {
                                                    **slot == Some(consumed_entity)
                                                })
                                            {
                                                if let Some(item_material) =
                                                    crd.material.get(item.unwrap())
                                                //This may cause issues
                                                {
                                                    if fulfills_material_requirements(
                                                        item_material,
                                                        tile_type.get_build_requirements(),
                                                    ) {
                                                        // Actually do it
                                                        twld.get_mut((x, y)).unwrap().tile = Tile {
                                                            seed: thread_rng().gen::<usize>(),
                                                            fertility: chunk_tile.tile.fertility,
                                                            tile_type,
                                                            tile_variant:
                                                                TileVariant::get_from_neighbours(
                                                                    twld.get_neighbours((x, y)),
                                                                ),
                                                        };

                                                        twld.refresh_tile_and_adjacent_variants((
                                                            x, y,
                                                        ));

                                                        // tile.tile_type = *tile_type;
                                                        inv.items[item_index] = None;

                                                        eids.delete(consumed_entity).unwrap();
                                                        // If entity is adjacent, despawn from entity map
                                                    } else {
                                                        println!("Entity attempting to build with items that do not fulfill the material requirements");
                                                    }
                                                } else {
                                                    println!("Entity attempting to build with items that do not have material components");
                                                }
                                            } else {
                                                println!("Entity attempting to build despite having no inventory");
                                            }
                                        } else {
                                            println!(
                                            "Entity attempting to build on an inappropriate tile!"
                                        );
                                        }
                                    } else {
                                        println!("Entity attempting to build on an unloaded tile!");
                                    }
                                } else {
                                    println!(
                                        "Entity attempting to build on a tile it cannot reach!"
                                    );
                                }
                            } else {
                                println!("Entity attempting to build despite having no position!");
                            }
                        } else {
                            println!("Entity attempting to build in unloaded tile");
                        }

                        //TODO:
                        //Check that the consumed entity is one of:
                        //-Adjacent
                        //-Held
                        //-In inventory
                        //And mark this somehow
                        //Then when crafting:
                        //Remove the entity from where it's stored
                        //Place the tile
                        None
                    }

                    AIAction::Craft {
                        recipe,
                        ingredients,
                    } => {
                        if let Some(ent_pos) = pos.get(eid) {
                            if let Some(inv) = inv.get_mut(eid) {
                                match recipe.craft(&ingredients, &lup, &eids, &crd) {
                                    Ok(crafted_entity) => {
                                        for item in ingredients {
                                            inv.remove(item);
                                            eids.delete(item).unwrap();
                                        }

                                        twld.spawn_entity(
                                            crafted_entity,
                                            (ent_pos.x, ent_pos.y),
                                            &mut pos,
                                        );
                                    }

                                    Err(err) => println!("Crafting error: {}", err),
                                }
                            } else {
                                println!("Entity attempted to craft without inventory");
                            }
                        } else {
                            println!("Entity attempted to craft without position");
                        }

                        None
                    }
                }
            }
        }
    }
}
