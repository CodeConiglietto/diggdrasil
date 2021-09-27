use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct AttackComponent {
    pub attack_roll: DiceRoll,
    pub attack_type: AttackType,
}
