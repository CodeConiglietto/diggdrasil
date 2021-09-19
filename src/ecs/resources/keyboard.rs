use ggez::event::KeyCode;

#[derive(Default)]
pub struct KeyboardResource {
    pub last_pressed_key: Option<KeyCode>,
}
