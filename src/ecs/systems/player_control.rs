use ggez::event::KeyCode;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::prelude::*;

pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        Read<'a, KeyboardResource>,
        ReadStorage<'a, PlayerControlComponent>,
        WriteStorage<'a, AIGoalComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (kb, pcc, mut gol) = data;

        for (_pcc, gol) in (&pcc, &mut gol).join() {
            println!("Key pressed: {:?}", kb.last_pressed_key);
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
                    _ => (),
                }
            }
        }
    }
}
