use serde::{Deserialize, Serialize};
use specs::{Entity, Component, VecStorage};

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct AIPerceptionComponent {
    #[serde(skip)]
    pub threats: Vec<Entity>,
    #[serde(skip)]
    pub food: Vec<Entity>, 
}
