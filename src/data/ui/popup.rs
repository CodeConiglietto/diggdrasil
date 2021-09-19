use ggez::event::{KeyCode, KeyMods};
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::prelude::*;

pub struct Popup {
    pub heading: String,
    pub available_goals: Vec<(usize, AIGoal)>,//TODO: make popup option struct which contain both an index and a goal in a named structure
    pub state: PopupState,
}

impl Popup {
    pub fn handle_input(&mut self, keycode: KeyCode, keymods: KeyMods) {
        if keycode == KeyCode::Escape {
            self.state = PopupState::Canceling;
        } else {
            if let Some(key_index) = key_to_index(keycode, keymods) {
                for i in 0..self.available_goals.len() {
                    if self.available_goals[i].0 == key_index {
                        self.state = PopupState::Returning(self.available_goals[i].1.clone());
                        break;
                    }
                }
            }
        }
    }

    pub fn render(&self, frame: &mut Frame<Ui>, size: Rect, data: &RenderData) {
        let list = List::new(
            self.available_goals
                .iter()
                .map(|(index, goal)| {
                    let c = index_to_letter(*index).unwrap();

                    let text = goal.get_textual_representation(data);

                    ListItem::new(format!("{}) {}", c, text))
                })
                .collect::<Vec<_>>(),
        );

        let block = Block::default()
            .title(self.heading.as_str())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));

        frame.render_widget(list.block(block), size);
    }
}
