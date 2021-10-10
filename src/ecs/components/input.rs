use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct InputComponent {
    pub popup: Option<Popup>,
    pub path: Option<Vec<IPosition>>,
}
