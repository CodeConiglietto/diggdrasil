use ggez::event::{KeyCode, KeyMods};

#[derive(Default)]
pub struct KeyboardResource {
    pub last_pressed_key: Option<KeyCode>,
    pub modifiers: KeyMods,
}
