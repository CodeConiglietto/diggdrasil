use rand::prelude::*;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

use crate::prelude::*;

pub struct ActionResolutionSystem;

impl<'a> System<'a> for ActionResolutionSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, TileMapResource>,
        Write<'a, EntityMapResource>,
        ReadStorage<'a, MaterialComponent>,
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, AIActionComponent>,
        WriteStorage<'a, IntendedMovementComponent>,
        WriteStorage<'a, InventoryComponent>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, DigestionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, mut tmap, mut emap, mat, mut pos, mut act, mut imc, mut inv, mut hpc, mut dig) =
            data;

        for (eid, act, imc) in (&eids, &mut act, &mut imc).join() {
            let current_action = &act.current_action;

            if let Some(action) = current_action {
                match action {
                    AIAction::MoveInDirection { x, y } => {
                        imc.x_delta = *x;
                        imc.y_delta = *y;
                        imc.controlled = true;
                    }
                    AIAction::AttackEntity { target } => {
                        //Will crash if attempting to attack a target without a position
                        let target_pos = pos.get(*target).unwrap();
                        let this_pos = pos.get(eid).unwrap();

                        if pos_is_adjacent((this_pos.x, this_pos.y), (target_pos.x, target_pos.y)) {
                            //Will crash if attempting to attack a target that has no health component
                            if let Some(target_hp) = &mut hpc.get_mut(*target) {
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
                    }
                    AIAction::PickUpItem { item } => {
                        if let Some(inventory) = inv.get_mut(eid) {
                            if let Some(entity_position) = pos.get(eid) {
                                if let Some(item_position) = pos.get(*item) {
                                    if entity_position.x == item_position.x
                                        && entity_position.y == item_position.y
                                    {
                                        if inventory.insert(*item) {
                                            emap.despawn_entity(*item, &mut pos);
                                        }
                                    } else {
                                        println!("Entity attempted to pick up item that it cannot reach!");
                                    }
                                } else {
                                    println!(
                                        "Entity attempting to pick up item that has no position!"
                                    );
                                }
                            } else {
                                println!(
                                    "Entity attempting to pick up item despite having no position!"
                                );
                            }
                        } else {
                            println!("No inventory to store item in!");
                        }
                    }
                    AIAction::DropItem { item } => {
                        if let Some(inventory) = inv.get_mut(eid) {
                            if let Some(entity_position) = pos.get(eid) {
                                inventory.remove(*item);
                                emap.spawn_entity(
                                    *item,
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
                    }
                    AIAction::EatItem { item } => {
                        if let Some(dig) = dig.get_mut(eid) {
                            if let Some(inv) = inv.get_mut(eid) {
                                inv.remove(*item);
                                dig.insert(*item);
                            }
                        }
                    }
                    AIAction::BuildAtLocation {
                        x,
                        y,
                        tile_type,
                        consumed_entity,
                    } => {
                        if let Some(pos) = pos.get(eid) {
                            if pos_is_adjacent((*x, *y), (pos.x, pos.y)) {
                                let tile_neighbour_types =
                                    tmap.get_neighbours(*x as usize, *y as usize);
                                let tile = &mut tmap.contents[[*x as usize, *y as usize]];

                                if tile.tile_type.available_buildings().contains(tile_type) {
                                    if let Some(inv) = inv.get_mut(eid) {
                                        if let Some((item_index, item)) = inv
                                            .items
                                            .iter()
                                            .enumerate()
                                            .find(|(_, slot)| **slot == Some(*consumed_entity))
                                        {
                                            if let Some(item_material) = mat.get(item.unwrap())
                                            //This may cause issues
                                            {
                                                if fulfills_material_requirements(
                                                    item_material,
                                                    tile_type.get_build_requirements(),
                                                ) {
                                                    // Actually do it
                                                    *tile = Tile {
                                                        seed: thread_rng().gen::<usize>(),
                                                        tile_type: *tile_type,
                                                        tile_variant:
                                                            TileVariant::get_from_neighbours(
                                                                tile_neighbour_types,
                                                            ),
                                                    };

                                                    tmap.refresh_tile_and_adjacent_variants(
                                                        *x as usize,
                                                        *y as usize,
                                                    );

                                                    // tile.tile_type = *tile_type;
                                                    inv.items[item_index] = None;

                                                    eids.delete(*consumed_entity).unwrap();
                                                    // If entity is adjacent, despawn from entity map
                                                } else {
                                                    println!("Entity attempting to build with items that do not fulfill the material requirements");
                                                }
                                            } else {
                                                println!("Entity attempting to build with items that do not have material components");
                                            }
                                        } else {
                                            println!("Entity attempting to build with items it doesn't have");
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
                                println!("Entity attempting to build on a tile it cannot reach!");
                            }
                        } else {
                            println!("Entity attempting to build despite having no position!");
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
                    }

                    AIAction::Craft {
                        recipe,
                        ingredients,
                    } => {
                        todo!();
                    }
                }
            }

            act.current_action = None;
        }
    }
}
