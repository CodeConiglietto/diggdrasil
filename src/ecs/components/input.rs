use specs::{Component, VecStorage};
use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct InputComponent { 
    pub popup: Option<Popup> 
}
