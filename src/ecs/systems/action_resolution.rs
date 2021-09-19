use specs::{Join, ReadStorage, System, Write, WriteStorage};

use crate::prelude::*;

pub struct ActionResolutionSystem;

impl<'a> System<'a> for ActionResolutionSystem {
    type SystemData = (
        Write<'a, ParticleMapResource>,
        ReadStorage<'a, PositionComponent>,
        WriteStorage<'a, AIActionComponent>,
        WriteStorage<'a, IntendedMovementComponent>,
        WriteStorage<'a, HealthComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pmap, pos, mut act, mut imc, mut hpc) = data;

        for (epos, act, imc) in (&pos, &mut act, &mut imc).join() {
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

                        if pos_is_adjacent((epos.x, epos.y), (target_pos.x, target_pos.y)) {
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
                }
            }

            act.current_action = None;
        }
    }
}
