use std::{convert::TryFrom, io::Error};

use bunnyfont::{
    char_transforms::{CharMirror, CharRotation},
    ggez::{GgBunnyChar, GgBunnyFontBatch},
};
use ggez::{
    graphics::{BlendMode, Color as GgColor, DrawParam, Drawable, Rect as GgRect},
    Context, GameResult,
};
use log::warn;
use ndarray::Array2;
use tui::{backend::Backend, buffer::Cell, layout::Rect as TuiRect, style::Color as TuiColor};

pub struct Ui {
    font_batch: GgBunnyFontBatch,
    buffer: Array2<Option<GgBunnyChar>>,
    scaling: f32,
}

impl Ui {
    pub fn new(
        font_batch: GgBunnyFontBatch,
        (width, height): (usize, usize),
        scaling: f32,
    ) -> Self {
        Self {
            font_batch,
            buffer: Array2::default((height, width)),
            scaling,
        }
    }
}

const UNKNOWN_CHAR_INDEX: usize = 0x03F;

impl Backend for Ui {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            let tui_char = cell.symbol.chars().next().unwrap();

            let (index, mirror, rotation) = match tui_char {
                '←' => (0x16B, CharMirror::None, CharRotation::None),
                '→' => (0x16C, CharMirror::None, CharRotation::None),
                '↓' => (0x16D, CharMirror::None, CharRotation::None),
                '↑' => (0x16E, CharMirror::None, CharRotation::None),

                // TODO Add when we have chars for them
                // '↖' => todo!(),
                // '↗' => todo!(),
                // '↘' => todo!(),
                // '↙' => todo!(),
                '┌' => (0x0DA, CharMirror::None, CharRotation::None),
                '─' => (0x0C4, CharMirror::None, CharRotation::None),
                '┐' => (0x0BF, CharMirror::None, CharRotation::None),
                '│' => (0x0B3, CharMirror::None, CharRotation::None),
                '└' => (0x0C0, CharMirror::None, CharRotation::None),
                '┘' => (0x0D9, CharMirror::None, CharRotation::None),

                '█' => (0x2B7, CharMirror::None, CharRotation::None),
                '▉' => (0x2B6, CharMirror::None, CharRotation::None),
                '▊' => (0x2B5, CharMirror::None, CharRotation::None),
                '▋' => (0x2B4, CharMirror::None, CharRotation::None),
                '▌' => (0x2B3, CharMirror::None, CharRotation::None),
                '▍' => (0x2B2, CharMirror::None, CharRotation::None),
                '▎' => (0x2B1, CharMirror::None, CharRotation::None),
                '▏' => (0x2B0, CharMirror::None, CharRotation::None),

                ' ' => (0x000, CharMirror::None, CharRotation::None),

                _ => {
                    if tui_char.is_ascii_graphic() {
                        (
                            usize::try_from(u32::from(tui_char)).unwrap(),
                            CharMirror::None,
                            CharRotation::None,
                        )
                    } else {
                        warn!(
                            "Unknown UTF-8 character mapping: {:?} (0x{:04X})",
                            tui_char,
                            u32::from(tui_char),
                        );

                        (UNKNOWN_CHAR_INDEX, CharMirror::None, CharRotation::None)
                    }
                }
            };

            let c = GgBunnyChar {
                index,
                foreground: convert_color(cell.fg).unwrap_or(GgColor::WHITE),
                background: convert_color(cell.bg),
                mirror,
                rotation,
            };

            self.buffer[[usize::from(y), usize::from(x)]] = Some(c);
        }

        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn get_cursor(&mut self) -> Result<(u16, u16), Error> {
        unimplemented!()
    }

    fn set_cursor(&mut self, _x: u16, _y: u16) -> Result<(), Error> {
        unimplemented!()
    }

    fn clear(&mut self) -> Result<(), Error> {
        self.buffer.fill(None);

        Ok(())
    }

    fn size(&self) -> Result<TuiRect, Error> {
        let (height, width) = self.buffer.dim();

        Ok(TuiRect::new(
            0,
            0,
            u16::try_from(width).unwrap(),
            u16::try_from(height).unwrap(),
        ))
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.font_batch.clear();

        for ((y, x), c) in self.buffer.indexed_iter() {
            if let Some(c) = c {
                c.draw_to_font_batch(
                    &mut self.font_batch,
                    (i32::try_from(x).unwrap(), i32::try_from(y).unwrap()),
                    self.scaling,
                );
            }
        }

        Ok(())
    }
}

fn convert_color(tui_color: TuiColor) -> Option<GgColor> {
    match tui_color {
        TuiColor::Reset => None,
        TuiColor::Black => Some(GgColor::BLACK),
        TuiColor::Red => Some(GgColor::RED),
        TuiColor::Green => Some(GgColor::GREEN),
        TuiColor::Yellow => Some(GgColor::YELLOW),
        TuiColor::Blue => Some(GgColor::BLUE),
        TuiColor::Magenta => Some(GgColor::MAGENTA),
        TuiColor::Cyan => Some(GgColor::CYAN),
        TuiColor::White => Some(GgColor::WHITE),

        // Semi-arbitrary color mappings
        TuiColor::Gray => Some(GgColor::from_rgb(190, 190, 190)),
        TuiColor::DarkGray => Some(GgColor::from_rgb(84, 84, 84)),
        TuiColor::LightRed => Some(GgColor::from_rgb(255, 84, 84)),
        TuiColor::LightGreen => Some(GgColor::from_rgb(84, 255, 84)),
        TuiColor::LightYellow => Some(GgColor::from_rgb(255, 255, 84)),
        TuiColor::LightBlue => Some(GgColor::from_rgb(84, 84, 255)),
        TuiColor::LightMagenta => Some(GgColor::from_rgb(255, 84, 255)),
        TuiColor::LightCyan => Some(GgColor::from_rgb(84, 255, 255)),

        TuiColor::Rgb(r, g, b) => Some(GgColor::from_rgb(r, g, b)),
        TuiColor::Indexed(_) => unimplemented!(),
    }
}

impl Drawable for Ui {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.font_batch.draw(ctx, param)
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<GgRect> {
        self.font_batch.dimensions(ctx)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.font_batch.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.font_batch.blend_mode()
    }
}
