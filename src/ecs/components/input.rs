use crate::prelude::*;
use specs::{Component, VecStorage};

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct InputComponent {
    pub popup: Option<Popup>,
    pub path: Option<Vec<(i32, i32)>>,
}
