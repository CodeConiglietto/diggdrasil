use serde::{Deserialize, Serialize};

use crate::prelude::*;

//TODO: make a builder for attacks, and move simple variants to that
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
    /// Ex. Twirl with a halberd all the way around your position, for 8 tiles
    Twirl,
    /// Any sort of stabbing or thrusting attack
    /// Provide the amount of tiles that the attack will travel
    LinearAttack { thrust_range: u32 },
    /// Any sort of swing attack
    /// Provide the length of swing in tiles
    RotationalAttack { swing_angle: u32 },
}

impl AttackType {
    pub fn get_offsets(
        &self,
        attack_direction: &Direction,
        swing_direction: Option<RotationDirection>,
    ) -> Vec<(i32, i32)> {
        match self {
            Self::Stab => Self::LinearAttack { thrust_range: 1 }
                .get_offsets(attack_direction, swing_direction),
            Self::Thrust => Self::LinearAttack { thrust_range: 2 }
                .get_offsets(attack_direction, swing_direction),
            Self::Slash => Self::RotationalAttack { swing_angle: 2 }
                .get_offsets(attack_direction, swing_direction),
            Self::Swing => Self::RotationalAttack { swing_angle: 3 }
                .get_offsets(attack_direction, swing_direction),
            Self::Twirl => Self::RotationalAttack { swing_angle: 8 }
                .get_offsets(attack_direction, swing_direction),
            Self::LinearAttack { thrust_range } => {
                let (adx, ady) = attack_direction.get_offset();

                (1..(*thrust_range + 1) as i32)
                    .rev()
                    .map(|i| (adx * i, ady * i))
                    .collect()
            }
            Self::RotationalAttack { swing_angle } => {
                //Will crash if the player attempts to attack themselves
                //This is fine for the moment
                let angle = attack_direction.get_angle().unwrap();

                let swing_direction =
                    swing_direction.unwrap_or_else(|| RotationDirection::get_random());

                (0..*swing_angle as i32)
                    .rev()
                    .map(|i| {
                        let angle = angle
                            + if swing_direction == RotationDirection::CounterClockwise {
                                8 - i
                            } else {
                                i
                            };

                        (IWave::Sin.get_value(angle), IWave::Cos.get_value(angle))
                    })
                    .collect()
            }
        }
    }
}
