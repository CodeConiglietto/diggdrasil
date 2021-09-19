use bunnyfont::traits::color::Color as BunnyColor;

#[derive(Clone, Copy)]
pub struct DiggColor {
    pub inner: ggez::graphics::Color,
}

impl BunnyColor for DiggColor {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        DiggColor {
            inner: ggez::graphics::Color::new(r, g, b, a),
        }
    }
}
