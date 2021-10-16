use serde::{Deserialize, Serialize};
use specs::{Component, Entity, VecStorage};

#[derive(Clone, Default, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct AIPerceptionComponent {
    #[serde(skip)]
    pub all: Vec<Entity>,
    #[serde(skip)]
    pub allies: Vec<Entity>,
    #[serde(skip)]
    pub threats: Vec<Entity>,
    #[serde(skip)]
    pub food: Vec<Entity>,
}
