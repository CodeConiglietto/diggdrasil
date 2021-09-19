use crate::prelude::*;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct AIActionComponent {
    pub current_action: Option<AIAction>,
}
