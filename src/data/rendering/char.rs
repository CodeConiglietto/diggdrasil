use bunnyfont::{char::BunnyChar, font::BunnyFont};

use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Rect};
use glam::*;

use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct DiggChar {
    pub inner: BunnyChar<DiggColor>,
}

impl DiggChar {
    pub fn draw_to_spritebatch(
        &self,
        (x, y): (i32, i32),
        font: &BunnyFont<DiggTexture>,
        sprite_batch: &mut SpriteBatch,
        render_scale: f32,
    ) {
        let bunny_char = &self.inner;

        let (x_scale, y_scale) = font.char_dimensions();
        let (x_scale, y_scale) = (x_scale as f32 * render_scale, y_scale as f32 * render_scale);

        if let Some(background) = &bunny_char.background {
            let (cx, cy, cw, ch) = font.get_src_uvs(0x2C7);
            let src = Rect::new(cx, cy, cw, ch);

            sprite_batch.add(
                DrawParam::new()
                    .color(background.inner)
                    .src(src)
                    .scale(Vec2::new(render_scale, render_scale))
                    .dest(Vec2::new(x_scale * x as f32, y_scale * y as f32)),
            );
        }

        let (cx, cy, cw, ch) = font.get_src_uvs(bunny_char.index);
        let src = Rect::new(cx, cy, cw, ch);

        sprite_batch.add(
            DrawParam::new()
                .color(bunny_char.foreground.inner)
                .src(src)
                .scale(Vec2::new(render_scale, render_scale))
                .dest(Vec2::new(x_scale * x as f32, y_scale * y as f32)),
        );
    }
}
