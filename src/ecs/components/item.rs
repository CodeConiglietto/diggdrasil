use serde::{Deserialize, Serialize};
use specs::{Component, NullStorage};

//A marker component to indicate that an entity can be stored in an inventory
//To be removed in future when more dynamic rules for picking up entities are implemented
#[derive(Clone, Default, Component, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct ItemComponent;
