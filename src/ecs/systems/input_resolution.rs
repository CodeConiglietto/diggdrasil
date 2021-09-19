use ggez::event::KeyCode;
use specs::{Join, System};

use crate::prelude::*;

pub struct InputResolutionSystem;

impl<'a> System<'a> for InputResolutionSystem {
    type SystemData = InputData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        //Entities
        let eids = data.entities;

        //Resources
        let emap = data.entity_map;
        let kb = data.keyboard;

        //Readable components
        let pos = data.position;
        let itc = data.item;

        //Writable components
        let mut gol = data.ai_goal;
        let mut inc = data.input;
        let mut inv = data.inventory;

        for (eid, pos, inc, gol) in (&eids, &pos, &mut inc, &mut gol).join() {
            println!("Key pressed: {:?}", kb.last_pressed_key);
            //skip input if player already has goal that they are completing.
            //Add keypress to interrupt player goal (space?)

            //deal with popup
            //pass keyboard to popup

            if let Some(key) = kb.last_pressed_key {
                if let Some(popup) = &mut inc.popup {
                    popup.handle_input(key);

                    match &popup.state {
                        PopupState::Waiting => {}
                        PopupState::Canceling => inc.popup = None,
                        PopupState::Returning(goal) => {
                            gol.current_goal = Some(goal.clone());
                            inc.popup = None;
                        }
                    }
                } else {
                    //if no popup
                    match key {
                        //TODO: move these to use direction enum
                        KeyCode::Numpad1 => {
                            gol.current_goal = Some(AIGoal::MoveInDirection { x: -1, y: 1 })
                        }
                        KeyCode::Numpad2 | KeyCode::Down => {
                            gol.current_goal = Some(AIGoal::MoveInDirection { x: 0, y: 1 })
                        }
                        KeyCode::Numpad3 => {
                            gol.current_goal = Some(AIGoal::MoveInDirection { x: 1, y: 1 })
                        }
                        KeyCode::Numpad4 | KeyCode::Left => {
                            gol.current_goal = Some(AIGoal::MoveInDirection { x: -1, y: 0 })
                        }
                        KeyCode::Numpad6 | KeyCode::Right => {
                            gol.current_goal = Some(AIGoal::MoveInDirection { x: 1, y: 0 })
                        }
                        KeyCode::Numpad7 => {
                            gol.current_goal = Some(AIGoal::MoveInDirection { x: -1, y: -1 })
                        }
                        KeyCode::Numpad8 | KeyCode::Up => {
                            gol.current_goal = Some(AIGoal::MoveInDirection { x: 0, y: -1 })
                        }
                        KeyCode::Numpad9 => {
                            gol.current_goal = Some(AIGoal::MoveInDirection { x: 1, y: -1 })
                        }
                        //TODO: add modifier check to see if player presses G or g.
                        //G picks up an entity in a manipulator
                        //g places an entity in the inventory
                        //Both actions require a manipulator
                        KeyCode::G => {
                            let PositionComponent { x, y } = pos;
                            let mut pickup_goals: Vec<_> = emap.contents
                                [[*x as usize, *y as usize]]
                            .iter()
                            .filter(|entity| itc.get(**entity).is_some())
                            .map(|item| AIGoal::PickUpItem { item: *item })
                            .collect();

                            match pickup_goals.len() {
                                0 => {}
                                1 => gol.current_goal = Some(pickup_goals.remove(0)),
                                _ => {
                                    inc.popup = Some(Popup {
                                        heading: String::from("Pick up what?"),
                                        available_goals: pickup_goals,
                                        state: PopupState::Waiting,
                                    })
                                }
                            }
                        }
                        KeyCode::D => {
                            let PositionComponent { x, y } = pos;
                            let mut drop_goals: Vec<_> = inv.get_mut(eid).unwrap().items
                                .iter()
                                .filter(|inventory_slot| 
                                    //Check that the inventory slot has something in it, and also that it is an item
                                    if let Some(item) = inventory_slot {
                                        itc.get(*item).is_some()
                                    } else {
                                        false
                                    }
                                )
                                .map(|item| AIGoal::DropItem { item: item.unwrap() })
                                .collect();

                            match drop_goals.len() {
                                0 => {}
                                _ => {
                                    inc.popup = Some(Popup {
                                        heading: String::from("Drop what?"),
                                        available_goals: drop_goals,
                                        state: PopupState::Waiting,
                                    })
                                }
                            }

                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
