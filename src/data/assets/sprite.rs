use bunnyfont::ggez::{GgBunnyChar, GgBunnyFontBatch};
use ndarray::prelude::*;

//TODO: in future make this a model, in 3d instead of 2d
//Then flatten into a 2d representation
pub struct Sprite {
    pub origin_x: i32,
    pub origin_y: i32,
    pub contents: Array2<GgBunnyChar>,
}

impl Sprite {
    pub fn draw_to_font_batch(
        &self,
        font_batch: &mut GgBunnyFontBatch,
        (x, y): (i32, i32),
        render_scale: f32,
    ) {
        let (width, height) = self.contents.dim();

        for sprite_x in 0..width {
            for sprite_y in 0..height {
                let draw_char = &self.contents[[sprite_x, sprite_y]];
                let (dest_x, dest_y) = (
                    sprite_y as i32 + x - self.origin_x,
                    sprite_x as i32 + y - self.origin_y,
                );

                draw_char.draw_to_font_batch(font_batch, (dest_x, dest_y), render_scale);
            }
        }
    }
}
