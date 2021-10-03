use ggez::event::KeyCode;
use log::debug;
use specs::{Join, System};

use crate::prelude::*;

pub struct InputResolutionSystem;

impl<'a> System<'a> for InputResolutionSystem {
    type SystemData = InputData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        //Entities
        let eids = data.entities;

        //Resources
        let twld = data.tile_world;
        let kb = data.keyboard;
        let ms = data.mouse;

        //Readable components
        let pos = data.position;
        let itc = data.item;

        //Writable components
        let mut gol = data.ai_goal;
        let mut pth = data.pathing;
        let mut inc = data.input;
        let mut inv = data.inventory;

        for (eid, pos, inc, gol, pth) in
            (&eids, &pos, &mut inc, &mut gol, (&mut pth).maybe()).join()
        {
            debug!("Key pressed: {:?}", kb.last_pressed_key);
            //skip input if player already has goal that they are completing.
            //Add keypress to interrupt player goal (space?)

            //deal with popup
            //pass keyboard to popup

            if let Some(key) = kb.last_pressed_key {
                if let Some(popup) = &mut inc.popup {
                    popup.handle_input(key, kb.modifiers);

                    match &popup.state {
                        PopupState::Waiting => {}
                        PopupState::Canceling => inc.popup = None,
                        PopupState::Returning(goal) => {
                            gol.goal_stack.push(goal.clone());
                            inc.popup = None;
                        }
                    }
                } else {
                    //if no popup
                    match key {
                        //TODO: move these to use direction enum
                        KeyCode::Numpad1 => {
                            gol.goal_stack.push(AIGoal::MoveInDirection {
                                direction: Direction::DownLeft,
                            });
                        }
                        KeyCode::Numpad2 | KeyCode::Down => {
                            gol.goal_stack.push(AIGoal::MoveInDirection {
                                direction: Direction::Down,
                            });
                        }
                        KeyCode::Numpad3 => {
                            gol.goal_stack.push(AIGoal::MoveInDirection {
                                direction: Direction::DownRight,
                            });
                        }
                        KeyCode::Numpad4 | KeyCode::Left => {
                            gol.goal_stack.push(AIGoal::MoveInDirection {
                                direction: Direction::Left,
                            });
                        }
                        KeyCode::Numpad6 | KeyCode::Right => {
                            gol.goal_stack.push(AIGoal::MoveInDirection {
                                direction: Direction::Right,
                            });
                        }
                        KeyCode::Numpad7 => {
                            gol.goal_stack.push(AIGoal::MoveInDirection {
                                direction: Direction::UpLeft,
                            });
                        }
                        KeyCode::Numpad8 | KeyCode::Up => {
                            gol.goal_stack.push(AIGoal::MoveInDirection {
                                direction: Direction::Up,
                            });
                        }
                        KeyCode::Numpad9 => {
                            gol.goal_stack.push(AIGoal::MoveInDirection {
                                direction: Direction::UpRight,
                            });
                        }
                        //TODO: add modifier check to see if player presses G or g.
                        //G picks up an entity in a manipulator
                        //g places an entity in the inventory
                        //Both actions require a manipulator
                        KeyCode::G => {
                            if let Some(inv) = inv.get(eid) {
                                if inv.any_slot_free() {
                                    let PositionComponent { x, y } = pos;
                                    let mut pickup_goals: Vec<_> = twld
                                        .get((*x, *y))
                                        .unwrap()
                                        .entities
                                        .iter()
                                        .filter(|entity| itc.get(**entity).is_some())
                                        .enumerate()
                                        .map(|(index, item)| {
                                            PopupListItem::new(
                                                index,
                                                None,
                                                AIGoal::StowItem { item: *item },
                                            )
                                        })
                                        .collect();

                                    match pickup_goals.len() {
                                        0 => {}
                                        1 => gol.goal_stack.push(pickup_goals.remove(0).goal),
                                        _ => {
                                            inc.popup = Some(Popup::list(
                                                String::from("Stow what?"),
                                                pickup_goals,
                                            ));
                                        }
                                    }
                                } else {
                                    debug!("No room in inventory!");
                                }
                            } else {
                                println!("No inventory to store item in!");
                            }
                        }
                        KeyCode::D => {
                            let drop_goals: Vec<_> = inv
                                .get_mut(eid)
                                .unwrap()
                                .items
                                .iter()
                                .enumerate()
                                .filter_map(|(index, inventory_slot)| {
                                    //Check that the inventory slot has something in it, and also that it is an item
                                    if let Some(item) = inventory_slot {
                                        if itc.get(*item).is_some() {
                                            return Some(PopupListItem::new(
                                                index,
                                                None,
                                                AIGoal::DropItem { item: *item },
                                            ));
                                        }
                                    }

                                    None
                                })
                                .collect();

                            match drop_goals.len() {
                                0 => {}
                                _ => {
                                    inc.popup =
                                        Some(Popup::list(String::from("Drop what?"), drop_goals));
                                }
                            }
                        }
                        KeyCode::W => {
                            let hold_goals: Vec<_> = inv
                                .get_mut(eid)
                                .unwrap()
                                .items
                                .iter()
                                .enumerate()
                                .filter_map(|(index, inventory_slot)| {
                                    //Check that the inventory slot has something in it, and also that it is an item
                                    if let Some(item) = inventory_slot {
                                        if itc.get(*item).is_some() {
                                            return Some(PopupListItem::new(
                                                index,
                                                None,
                                                AIGoal::HoldItem { item: Some(*item) },
                                            ));
                                        }
                                    }

                                    None
                                })
                                .collect();

                            match hold_goals.len() {
                                0 => {}
                                _ => {
                                    inc.popup =
                                        Some(Popup::list(String::from("Hold what?"), hold_goals));
                                }
                            }
                        }
                        KeyCode::E => {
                            gol.goal_stack.push(AIGoal::EatItem { item: None });
                        }
                        KeyCode::B => {
                            //TODO: ensure player has some way to manipulate objects, otherwise they can't build :(
                            let pos_x = pos.x;
                            let pos_y = pos.y;

                            inc.popup = Some(Popup::directions(
                                String::from("Build where?"),
                                Directions::all(),
                                move |dir| {
                                    let (dx, dy) = dir.get_offset();

                                    AIGoal::Build {
                                        x: pos_x + dx,
                                        y: pos_y + dy,
                                        tile_type: None,
                                        consumed_entity: None,
                                    }
                                },
                            ));
                        }
                        KeyCode::C => gol.goal_stack.push(AIGoal::Craft {
                            recipe: None,
                            ingredients: Vec::new(),
                        }),
                        _ => (),
                    }
                }
            }

            let left = pos.x - MAP_X_SIZE as i32 / 2;
            let top = pos.y - MAP_Y_SIZE as i32 / 2;

            let (mouse_x, mouse_y) = ms.position;
            let (char_mouse_x, char_mouse_y) = (
                (mouse_x / (RENDER_SCALE * 8.0)).floor() as i32,
                (mouse_y / (RENDER_SCALE * 8.0)).floor() as i32,
            );

            if (0..MAP_X_SIZE as i32).contains(&char_mouse_x)
                && (0..MAP_Y_SIZE as i32).contains(&char_mouse_y)
            {
                let (tile_mouse_x, tile_mouse_y) = (char_mouse_x + left, char_mouse_y + top);

                if let Some(pth) = pth {
                    inc.path = pth.pathfind(&*twld, (pos.x, pos.y), (tile_mouse_x, tile_mouse_y));
                }
            }

            if ms.left_button_pressed {
                if let Some(path) = &inc.path {
                    gol.goal_stack
                        .push(AIGoal::TravelPath { path: path.clone() });
                }
            }
        }
    }
}
