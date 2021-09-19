use crate::prelude::*;
use bunnyfont::traits::source_image::SourceImage;

pub struct DiggTexture {
    pub inner: ggez::graphics::Image,
}

impl SourceImage for DiggTexture {
    type Color = DiggColor;

    fn get_pixel_dimensions(&self) -> (usize, usize) {
        (self.inner.width() as usize, self.inner.height() as usize)
    }
}
