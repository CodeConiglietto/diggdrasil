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
    pub popup_type: PopupType,
    pub state: PopupState,
}

impl Popup {
    pub fn list(heading: String, goals: Vec<PopupListItem>) -> Self {
        Self {
            heading,
            popup_type: PopupType::List { goals },
            state: PopupState::Waiting,
        }
    }

    pub fn directions<F>(heading: String, directions: Directions, f: F) -> Self
    where
        F: FnOnce(Direction) -> AIGoal + Send + Sync + 'static,
    {
        Self {
            heading,
            popup_type: PopupType::Directions {
                directions,
                f: Some(Box::new(f)),
            },
            state: PopupState::Waiting,
        }
    }

    pub fn handle_input(&mut self, keycode: KeyCode, keymods: KeyMods) {
        if keycode == KeyCode::Escape {
            self.state = PopupState::Canceling;
            return;
        }

        match &mut self.popup_type {
            PopupType::List { goals } => {
                if let Some(key_index) = key_to_index(keycode, keymods) {
                    if let Some(selected) = goals.iter().find(|item| item.index == key_index) {
                        self.state = PopupState::Returning(selected.goal.clone());
                        return;
                    }
                }
            }

            PopupType::Directions { directions, f } => {
                if let Some(direction) = Direction::from_keycode(keycode) {
                    if directions.contains(direction.into()) {
                        self.state = PopupState::Returning(f.take().unwrap()(direction))
                    }
                }
            }
        }
    }

    pub fn render(&self, frame: &mut Frame<Ui>, size: Rect, data: &RenderData) {
        let block = Block::default()
            .title(self.heading.as_str())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));

        match &self.popup_type {
            PopupType::List { goals } => {
                let list = List::new(
                    goals
                        .iter()
                        .map(|item| {
                            let c = index_to_letter(item.index).unwrap();
                            let text = item.goal.get_textual_representation(data);

                            ListItem::new(format!("{}) {}", c, text))
                        })
                        .collect::<Vec<_>>(),
                );

                frame.render_widget(list.block(block), size);
            }

            PopupType::Directions { directions, .. } => {
                frame.render_widget(DirectionsWidget::new(*directions).block(block), size);
            }
        }
    }
}
