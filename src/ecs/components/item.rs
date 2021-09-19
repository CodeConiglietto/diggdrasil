use specs::{Component, VecStorage};

//A marker component to indicate that an entity can be stored in an inventory
//To be removed in future when more dynamic rules for picking up entities are implemented
#[derive(Component)]
#[storage(VecStorage)]
pub struct ItemComponent;
