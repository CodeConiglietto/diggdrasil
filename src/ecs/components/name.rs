use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct NameComponent {
    pub name: String,
}
