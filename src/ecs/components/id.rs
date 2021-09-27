use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Component, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct IdComponent {
    pub id: u64,
}
