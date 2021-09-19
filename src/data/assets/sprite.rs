use bunnyfont::font::BunnyFont;
use ggez::graphics::spritebatch::SpriteBatch;
use ndarray::prelude::*;

use crate::prelude::*;

//TODO: in future make this a model, in 3d instead of 2d
//Then flatten into a 2d representation
pub struct Sprite {
    pub origin_x: i32,
    pub origin_y: i32,
    pub contents: Array2<DiggChar>,
}

impl Sprite {
    pub fn draw_to_spritebatch(
        &self,
        (x, y): (i32, i32),
        font: &BunnyFont<DiggTexture>,
        sprite_batch: &mut SpriteBatch,
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

                draw_char.draw_to_spritebatch((dest_x, dest_y), font, sprite_batch, render_scale);
            }
        }
    }
}
