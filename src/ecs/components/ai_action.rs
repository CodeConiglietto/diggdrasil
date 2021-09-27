use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct AIActionComponent {
    #[serde(skip)]
    pub current_action: Option<AIAction>,
}
