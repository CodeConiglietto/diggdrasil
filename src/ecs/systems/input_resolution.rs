use ggez::event::KeyCode;
use specs::{Join, Read, System, WriteStorage};

use crate::prelude::*;

pub struct InputResolutionSystem;

impl<'a> System<'a> for InputResolutionSystem {
    type SystemData = InputData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        //Resources
        let kb = data.keyboard;
        let emap = data.entity_map;

        //Readable components
        let pos = data.position;
        let itc = data.item;

        //Writable components
        let mut inc = data.input;
        let mut gol = data.ai_goal;

        for (pos, inc, gol) in (&pos, &mut inc, &mut gol).join() {
            println!("Key pressed: {:?}", kb.last_pressed_key);
            //skip input if player already has goal that they are completing.
            //Add keypress to interrupt player goal (space?)

            //deal with popup
            //pass keyboard to popup

            //if no popup
            if let Some(key) = kb.last_pressed_key {
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
                        let PositionComponent{x, y} = pos;
                        let mut pickup_goals: Vec<_> = emap.contents[[*x as usize, *y as usize]].iter().filter(
                            |entity|
                            {
                                itc.get(**entity).is_some()
                            }
                        )
                        .map(
                            |item|
                            {
                                AIGoal::PickUpItem{ item: *item }
                            }
                        )
                        .collect();

                        if pickup_goals.len() == 1 {
                            gol.current_goal = Some(pickup_goals.remove(0));
                        }else{
                            inc.popup = Some(Popup{ heading: String::from("Pick up what?"), available_goals: pickup_goals, state: PopupState::Waiting });
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
