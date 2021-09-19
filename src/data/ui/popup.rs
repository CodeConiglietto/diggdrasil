use ggez::event::KeyCode;
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::prelude::*;

pub struct Popup {
    pub heading: String,
    pub available_goals: Vec<AIGoal>,
    pub state: PopupState,
}

impl Popup {
    pub fn handle_input(&mut self, key: KeyCode) {
        // TODO Make this a saner function
        match key {
            KeyCode::A => self.state = PopupState::Returning(self.available_goals[0].clone()),
            KeyCode::B => self.state = PopupState::Returning(self.available_goals[1].clone()),
            KeyCode::C => self.state = PopupState::Returning(self.available_goals[2].clone()),
            KeyCode::D => self.state = PopupState::Returning(self.available_goals[3].clone()),
            KeyCode::E => self.state = PopupState::Returning(self.available_goals[4].clone()),
            KeyCode::F => self.state = PopupState::Returning(self.available_goals[5].clone()),

            KeyCode::Escape => self.state = PopupState::Canceling,

            _ => {}
        }
    }

    pub fn render(&self, frame: &mut Frame<Ui>, size: Rect, data: &RenderData) {
        let list = List::new(
            self.available_goals
                .iter()
                .enumerate()
                .map(|(i, goal)| {
                    ListItem::new(format!(
                        "{}) {}",
                        char::from(u32::from('a') as u8 + i as u8), // TODO Change this to a sane function later
                        goal.get_textual_representation(data)
                    ))
                })
                .collect::<Vec<_>>(),
        );

        let block = Block::default()
            .title(self.heading.as_str())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));

        frame.render_widget(list.block(block), size);

        //a) Goal 1
        //b) Goal 2
    }
}
