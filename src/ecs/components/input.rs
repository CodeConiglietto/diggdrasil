use crate::prelude::*;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct InputComponent {
    pub popup: Option<Popup>,
}
