use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AttackType {
    /// Ex. Stab with a dagger towards the tile in front of you
    Stab,
    /// Ex. Thrust with a polearm in front of you, and the tile behind that
    Thrust,
    /// Ex. Slash with a sword in front of you, and also one tile that is adjacent to both you and that tile
    Slash,
    /// Ex. Swing with an axe in front of you, and follow through around your position for 3 tiles
    Swing,
    /// Ex. Twirl with a halberd all the way around your position, for 8 tile
    Twirl,
}

impl AttackType {
    pub fn get_affected_offsets(
        &self,
        attack_direction: Direction,
        swing_direction: Option<RotationDirection>,
    ) -> Vec<(i32, i32)> {
        match self {
            Self::Stab => vec![attack_direction.get_offset()],
            Self::Thrust => todo!(),
            _ => todo!(),
        }
    }
}
