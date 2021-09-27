use serde::{Deserialize, Serialize};
use specs::{Component, NullStorage};

#[derive(Clone, Default, Component, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct ColliderComponent;
