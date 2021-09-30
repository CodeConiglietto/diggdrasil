use bunnyfont::ggez::{GgBunnyChar, GgBunnyFontBatch};

pub struct Symbol {
    pub draw_chars: Vec<GgBunnyChar>,
}

impl Symbol {
    pub fn empty() -> Symbol {
        Symbol {
            draw_chars: Vec::new(),
        }
    }

    pub fn draw_to_font_batch(
        &self,
        font_batch: &mut GgBunnyFontBatch,
        (x, y): (i32, i32),
        render_scale: f32,
    ) {
        for draw_char in &self.draw_chars {
            draw_char.draw_to_font_batch(font_batch, (x, y), render_scale);
        }
    }
}
