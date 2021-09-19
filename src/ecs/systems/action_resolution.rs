use specs::{Entities, Join, System, Write, WriteStorage};

use crate::prelude::*;

pub struct ActionResolutionSystem;

impl<'a> System<'a> for ActionResolutionSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, EntityMapResource>,
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, AIActionComponent>,
        WriteStorage<'a, IntendedMovementComponent>,
        WriteStorage<'a, InventoryComponent>,
        WriteStorage<'a, HealthComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, mut emap, mut pos, mut act, mut imc, mut inv, mut hpc) = data;

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
                                    target_hp.value -= 1;
                                }
                            } else {
                                println!("Entity attempted to attack target that has no HP component!");
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
                                    println!("Entity attempting to pick up item that has no position!");
                                }
                            } else {
                                println!("Entity attempting to pick up item despite having no position!");
                            }
                        } else {
                            println!("No inventory to store item in!");
                        }
                    }
                    AIAction::DropItem { item } => {
                        if let Some(inventory) = inv.get_mut(eid) {
                            if let Some(entity_position) = pos.get(eid) {
                                inventory.remove(*item);
                                emap.spawn_entity(*item, (entity_position.x, entity_position.y), &mut pos);
                            } else {
                                println!("Entity attempting to drop an item despite having no position!");
                            }
                        } else {
                            println!("Entity attempting to drop an item despite having no inventory!");
                        }
                    }
                }
            }

            act.current_action = None;
        }
    }
}
