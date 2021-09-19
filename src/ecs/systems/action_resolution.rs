use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

use crate::prelude::*;

pub struct ActionResolutionSystem;

impl<'a> System<'a> for ActionResolutionSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, ParticleMapResource>,
        Write<'a, EntityMapResource>,
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, AIActionComponent>,
        WriteStorage<'a, IntendedMovementComponent>,
        WriteStorage<'a, InventoryComponent>,
        WriteStorage<'a, HealthComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, mut pmap, mut emap, mut pos, mut act, mut imc, mut inv, mut hpc) = data;

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
                                println!("Failed attack!");
                            }
                        }
                    }
                    AIAction::PickUpItem {item} => {
                        if let Some(inventory) = inv.get_mut(eid){
                            if let Some(entity_position) = pos.get(eid) {
                                if let Some(item_position) = pos.get(*item) {
                                    if entity_position.x == item_position.x && entity_position.y == item_position.y {
                                        inventory.insert(*item);
                                        emap.despawn_entity(*item, &mut pos);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            act.current_action = None;
        }
    }
}
