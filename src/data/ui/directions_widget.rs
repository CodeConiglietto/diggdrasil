use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};

use crate::prelude::*;

pub struct DirectionsWidget<'a> {
    directions: Directions,
    block: Option<Block<'a>>,
}

impl<'a> DirectionsWidget<'a> {
    pub fn new(directions: Directions) -> Self {
        Self {
            directions,
            block: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for DirectionsWidget<'a> {
    fn render(mut self, mut area: Rect, buf: &mut Buffer) {
        if let Some(block) = self.block.take() {
            let inner_area = block.inner(area);
            block.render(area, buf);
            area = inner_area;
        };

        let left = area.width.saturating_sub(5) / 2;
        let top = area.height.saturating_sub(5) /2;

        if self.directions.contains(Directions::UP_LEFT) {
            buf.get_mut(left + 1, top + 1).set_symbol("7");
            buf.get_mut(left, top).set_symbol("↖");
        }

        if self.directions.contains(Directions::UP) {
            buf.get_mut(left + 2, top + 1).set_symbol("8");
            buf.get_mut(left + 2, top).set_symbol("↑");
        }

        if self.directions.contains(Directions::UP_RIGHT) {
            buf.get_mut(left + 3, top + 1).set_symbol("9");
            buf.get_mut(left + 4, top).set_symbol("↗");
        }

        if self.directions.contains(Directions::LEFT) {
            buf.get_mut(left + 1, top + 2).set_symbol("4");
            buf.get_mut(left, top + 2).set_symbol("←");
        }

        if self.directions.contains(Directions::NONE) {
            buf.get_mut(left + 2, top + 2).set_symbol("5");
        }

        if self.directions.contains(Directions::RIGHT) {
            buf.get_mut(left + 3, top + 2).set_symbol("6");
            buf.get_mut(left + 4, top + 2).set_symbol("→");
        }

        if self.directions.contains(Directions::DOWN_LEFT) {
            buf.get_mut(left + 1, top + 3).set_symbol("1");
            buf.get_mut(left, top + 4).set_symbol("↙");
        }

        if self.directions.contains(Directions::DOWN) {
            buf.get_mut(left + 2, top + 3).set_symbol("2");
            buf.get_mut(left + 2, top + 4).set_symbol("↓");
        }

        if self.directions.contains(Directions::DOWN_RIGHT) {
            buf.get_mut(left + 3, top + 3).set_symbol("3");
            buf.get_mut(left + 4, top + 4).set_symbol("↘");
        }
    }
}
